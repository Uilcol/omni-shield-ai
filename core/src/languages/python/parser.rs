use tree_sitter::{Node, Parser};

pub struct PythonParser;

impl PythonParser {
    pub fn find_calls(code: &str, function_name: &str) -> Vec<usize> {
        let mut parser = Parser::new();

        parser
            .set_language(tree_sitter_python::language())
            .expect("failed to initialize Python tree-sitter language");

        let tree = parser
            .parse(code, None)
            .expect("failed to parse Python source");

        let mut lines = Vec::new();

        Self::walk_calls(tree.root_node(), code, function_name, &mut lines);

        lines
    }

    pub fn find_sql_injection_lines(code: &str) -> Vec<usize> {
        let mut parser = Parser::new();

        parser
            .set_language(tree_sitter_python::language())
            .expect("failed to load Python grammar");

        let tree = parser
            .parse(code, None)
            .expect("failed to parse Python source");

        let mut lines = Vec::new();
        let mut tainted_vars = std::collections::HashSet::new();

        // Discover local taint transitively:
        // source -> variable -> function return -> variable -> SQL sink.
        //
        // Iterate until no new tainted variable is discovered so chained
        // assignments and tainted function-return values are covered
        // deterministically.
        let tainted_return_functions = Self::find_tainted_return_functions(tree.root_node(), code);

        loop {
            let mut changed = false;

            for source_line in code.lines() {
                let trimmed = source_line.trim_start();

                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }

                if let Some(eq) = trimmed.find('=') {
                    let lhs = trimmed[..eq].trim();
                    let rhs = &trimmed[eq + 1..];

                    let valid_identifier = !lhs.is_empty()
                        && lhs.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
                        && lhs
                            .chars()
                            .next()
                            .map(|c| c.is_ascii_alphabetic() || c == '_')
                            .unwrap_or(false);

                    if !valid_identifier {
                        continue;
                    }

                    let rhs_code = Self::remove_string_literals(rhs);
                    let rhs_lower = rhs_code.to_ascii_lowercase();

                    let direct_source = Self::is_tainted_expression(rhs);

                    let propagated_source = tainted_vars.iter().any(|name| {
                        rhs_lower
                            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                            .any(|token| token == name)
                    });

                    let returned_taint = tainted_return_functions
                        .iter()
                        .any(|name| Self::contains_function_call(&rhs_lower, name));

                    if (direct_source || propagated_source || returned_taint)
                        && tainted_vars.insert(lhs.to_ascii_lowercase())
                    {
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }

        for (index, source_line) in code.lines().enumerate() {
            let line_number = index + 1;
            let trimmed = source_line.trim_start();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let upper = trimmed.to_ascii_uppercase();
            let lower = trimmed.to_ascii_lowercase();

            let has_sql = upper.contains("SELECT ")
                || upper.contains("INSERT ")
                || upper.contains("UPDATE ")
                || upper.contains("DELETE ");

            if !has_sql {
                continue;
            }

            let has_interpolation = trimmed.contains('{') && trimmed.contains('}');

            let has_f_string_prefix = lower.contains("f\"")
                || lower.contains("f'")
                || lower.contains("rf\"")
                || lower.contains("rf'")
                || lower.contains("fr\"")
                || lower.contains("fr'")
                || lower.contains("bf\"")
                || lower.contains("bf'")
                || lower.contains("fb\"")
                || lower.contains("fb'");

            if has_interpolation && has_f_string_prefix {
                lines.push(line_number);
                continue;
            }

            if !trimmed.contains('+') {
                continue;
            }

            let code_only = Self::remove_string_literals(trimmed);
            let code_only_lower = code_only.to_ascii_lowercase();

            let direct_taint_names = [
                "user_input",
                "request.get",
                "request.post",
                "input(",
                "argv",
                "params",
            ];

            let has_direct_taint = direct_taint_names
                .iter()
                .any(|name| code_only_lower.contains(name));

            let has_local_taint = tainted_vars.iter().any(|name| {
                code_only_lower
                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .any(|token| token == name)
            });

            let has_string_concatenation = trimmed.contains('"') && trimmed.contains('+')
                || trimmed.contains('\'') && trimmed.contains('+');

            if (has_direct_taint || has_local_taint) && has_string_concatenation {
                lines.push(line_number);
            }
        }

        Self::walk_sql_nodes(tree.root_node(), code, &mut lines);

        lines.sort_unstable();
        lines.dedup();
        lines
    }

    fn remove_string_literals(text: &str) -> String {
        let mut result = String::with_capacity(text.len());
        let mut chars = text.chars().peekable();
        let mut quote: Option<char> = None;
        let mut escaped = false;

        while let Some(ch) = chars.next() {
            if let Some(q) = quote {
                if escaped {
                    escaped = false;
                    continue;
                }

                if ch == '\\' {
                    escaped = true;
                    continue;
                }

                if ch == q {
                    quote = None;
                }

                continue;
            }

            if ch == '"' || ch == '\'' {
                quote = Some(ch);
                result.push(' ');
                continue;
            }

            result.push(ch);
        }

        result
    }

    fn is_tainted_expression(text: &str) -> bool {
        let code_only = Self::remove_string_literals(text);
        let lower = code_only.to_ascii_lowercase();

        lower.contains("input(")
            || lower.contains("request.get")
            || lower.contains("request.post")
            || lower.contains("argv")
            || lower.contains("params")
            || lower
                .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .any(|token| token == "user_input")
    }

    fn find_tainted_return_functions(root: Node, code: &str) -> std::collections::HashSet<String> {
        let mut functions = std::collections::HashSet::new();
        Self::walk_function_definitions(root, code, &mut functions);
        functions
    }

    fn walk_function_definitions(
        node: Node,
        code: &str,
        functions: &mut std::collections::HashSet<String>,
    ) {
        if node.kind() == "function_definition" {
            let name = node
                .child_by_field_name("name")
                .and_then(|n| n.utf8_text(code.as_bytes()).ok())
                .unwrap_or("");

            if !name.is_empty() {
                if let Some(body) = node.child_by_field_name("body") {
                    let body_text = body.utf8_text(code.as_bytes()).unwrap_or("");

                    let mut local_tainted = std::collections::HashSet::new();
                    let mut saw_tainted_return = false;

                    for line in body_text.lines() {
                        let trimmed = line.trim_start();

                        if trimmed.starts_with('#') || trimmed.is_empty() {
                            continue;
                        }

                        if let Some(rest) = trimmed.strip_prefix("return ") {
                            if Self::is_tainted_expression(rest) {
                                saw_tainted_return = true;
                            }

                            let code_only = Self::remove_string_literals(rest);
                            let lower = code_only.to_ascii_lowercase();

                            if local_tainted.iter().any(|var| {
                                lower
                                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                                    .any(|token| token == var)
                            }) {
                                saw_tainted_return = true;
                            }
                        }

                        if let Some(eq) = trimmed.find('=') {
                            let lhs = trimmed[..eq].trim();

                            let valid_identifier = !lhs.is_empty()
                                && lhs.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
                                && lhs
                                    .chars()
                                    .next()
                                    .map(|c| c.is_ascii_alphabetic() || c == '_')
                                    .unwrap_or(false);

                            if valid_identifier && Self::is_tainted_expression(&trimmed[eq + 1..]) {
                                local_tainted.insert(lhs.to_ascii_lowercase());
                            }
                        }
                    }

                    if saw_tainted_return {
                        functions.insert(name.to_ascii_lowercase());
                    }
                }
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_function_definitions(child, code, functions);
        }
    }

    fn contains_function_call(expression: &str, function_name: &str) -> bool {
        let needle = format!("{}(", function_name);
        expression.contains(&needle)
    }

    fn walk_sql_nodes(node: Node, code: &str, lines: &mut Vec<usize>) {
        if node.kind() == "binary_operator" {
            if let (Some(left), Some(right)) = (
                node.child_by_field_name("left"),
                node.child_by_field_name("right"),
            ) {
                let left_text = left.utf8_text(code.as_bytes()).unwrap_or("");
                let right_text = right.utf8_text(code.as_bytes()).unwrap_or("");
                let full_text = node.utf8_text(code.as_bytes()).unwrap_or("");

                if full_text.contains('+')
                    && ((Self::is_sql_literal(left_text)
                        && Self::is_tainted_expression(right_text))
                        || (Self::is_sql_literal(right_text)
                            && Self::is_tainted_expression(left_text)))
                {
                    lines.push(node.start_position().row + 1);
                }
            }
        }

        if node.kind() == "interpolation" || node.kind() == "formatted_string" {
            let text = node.utf8_text(code.as_bytes()).unwrap_or("");

            if text.to_ascii_uppercase().contains("SELECT ")
                && text.contains('{')
                && text.contains('}')
            {
                lines.push(node.start_position().row + 1);
            }
        }

        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            Self::walk_sql_nodes(child, code, lines);
        }
    }

    fn is_sql_literal(text: &str) -> bool {
        let upper = text.to_ascii_uppercase();

        upper.contains("SELECT ")
            || upper.contains("INSERT INTO ")
            || upper.contains("UPDATE ")
            || upper.contains("DELETE FROM ")
    }

    fn walk_calls(node: Node, code: &str, function_name: &str, lines: &mut Vec<usize>) {
        if node.kind() == "call" {
            if let Some(function_node) = node.child_by_field_name("function") {
                if let Ok(text) = function_node.utf8_text(code.as_bytes()) {
                    if text == function_name {
                        lines.push(function_node.start_position().row + 1);
                    }
                }
            }
        }

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            Self::walk_calls(child, code, function_name, lines);
        }
    }
}
