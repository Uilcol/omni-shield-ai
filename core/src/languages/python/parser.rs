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

        let _tree = parser
            .parse(code, None)
            .expect("failed to parse Python source");

        let mut lines = Vec::new();

        for (index, source_line) in code.lines().enumerate() {
            let line_number = index + 1;
            let trimmed = source_line.trim_start();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let upper = trimmed.to_ascii_uppercase();

            let has_sql = upper.contains("SELECT ")
                || upper.contains("INSERT ")
                || upper.contains("UPDATE ")
                || upper.contains("DELETE ");

            if !has_sql {
                continue;
            }

            let has_interpolation =
                trimmed.contains('{') && trimmed.contains('}');

            let lower = trimmed.to_ascii_lowercase();

            let has_f_string_prefix =
                lower.contains("f\"")
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

            let has_plus = trimmed.contains('+');

            if !has_plus {
                continue;
            }

            let taint_names = [
                "user_input",
                "request.get",
                "request.post",
                "input(",
                "argv",
                "params",
            ];

            let has_tainted_expression = taint_names
                .iter()
                .any(|name| lower.contains(name));

            let has_string_concatenation = trimmed.contains('+')
                && (trimmed.contains("\" +")
                    || trimmed.contains("' +")
                    || trimmed.contains("+ \"")
                    || trimmed.contains("+ '"));

            if has_tainted_expression && has_string_concatenation {
                lines.push(line_number);
            }
        }

        // Preserve the AST-based detector as an additional source of
        // precision when the grammar exposes binary operators normally.
        Self::walk_sql_nodes(_tree.root_node(), code, &mut lines);

        lines.sort_unstable();
        lines.dedup();
        lines
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
                    && ((Self::is_sql_literal(left_text) && Self::is_tainted_expression(right_text))
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

    fn is_tainted_expression(text: &str) -> bool {
        let trimmed = text.trim();

        trimmed.contains("user_input")
            || trimmed.contains("request.GET")
            || trimmed.contains("request.POST")
            || trimmed.contains("input(")
            || trimmed.contains("argv")
            || trimmed.contains("params")
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
