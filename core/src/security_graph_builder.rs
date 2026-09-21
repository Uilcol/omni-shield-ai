use crate::security::database::SecurityDatabase;
use crate::security_graph::{
    SecurityEdge, SecurityEdgeKind, SecurityGraph, SecurityNode, SecurityNodeKind,
};
use std::collections::HashSet;
use tree_sitter::{Node, Parser};

pub struct SecurityGraphBuilder;

impl SecurityGraphBuilder {
    pub fn find_python_source_sink_paths(
        code: &str,
        file: &str,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        let graph = Self::from_python(code, file);

        let sources: Vec<String> = graph
            .nodes()
            .filter(|node| node.kind == SecurityNodeKind::Source)
            .map(|node| node.id.clone())
            .collect();

        let sinks: Vec<String> = graph
            .nodes()
            .filter(|node| node.kind == SecurityNodeKind::Sink)
            .map(|node| node.id.clone())
            .collect();

        let mut paths = Vec::new();

        for source in &sources {
            for sink in &sinks {
                paths.extend(graph.source_to_sink_paths(source, sink, max_depth));
            }
        }

        paths.sort();
        paths.dedup();
        paths
    }

    pub fn from_python(code: &str, file: &str) -> SecurityGraph {
        let mut parser = Parser::new();

        parser
            .set_language(tree_sitter_python::language())
            .expect("failed to initialize Python tree-sitter language");

        let tree = parser
            .parse(code, None)
            .expect("failed to parse Python source");

        let mut graph = SecurityGraph::new();

        // Build structural nodes first.
        let mut context = BuildContext {
            code,
            file,
            graph: &mut graph,
            tainted_variables: HashSet::new(),
        };

        Self::walk(tree.root_node(), &mut context, None);
        drop(context);

        // Resolve cross-function data flow over the structural graph.
        Self::connect_interprocedural_edges(&mut graph, code, file);

        // Build deterministic source nodes and variable flow for assignments.
        let mut tainted_variables = HashSet::new();

        for (index, source_line) in code.lines().enumerate() {
            let line = index + 1;
            let trimmed = source_line.trim_start();

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let Some(eq) = trimmed.find('=') else {
                continue;
            };

            let lhs = trimmed[..eq].trim();
            let rhs = trimmed[eq + 1..].trim();

            if !Self::is_identifier(lhs) {
                continue;
            }

            let rhs_lower = rhs.to_ascii_lowercase();

            let direct_source = Self::contains_source(&rhs_lower);
            let inherited_source = tainted_variables.iter().any(|name| {
                rhs_lower
                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                    .any(|token| token == name)
            });

            if !(direct_source || inherited_source) {
                continue;
            }

            let variable_name = lhs.to_ascii_lowercase();
            tainted_variables.insert(variable_name.clone());

            let variable_id = format!("{file}:variable:{variable_name}:{line}");

            if graph.node(&variable_id).is_none() {
                graph.add_node(SecurityNode {
                    id: variable_id.clone(),
                    kind: SecurityNodeKind::Variable,
                    label: lhs.to_string(),
                    file: file.to_string(),
                    line,
                });
            }

            let source_id = format!("{file}:source:{variable_name}:{line}");

            if graph.node(&source_id).is_none() {
                graph.add_node(SecurityNode {
                    id: source_id.clone(),
                    kind: SecurityNodeKind::Source,
                    label: rhs.to_string(),
                    file: file.to_string(),
                    line,
                });
            }

            graph.add_edge(SecurityEdge {
                from: source_id,
                to: variable_id,
                kind: SecurityEdgeKind::FlowsTo,
            });
        }

        // Deterministically connect tainted variables to sink calls on the
        // same source line. This gives us an unambiguous:
        // Source -> Variable -> Call -> Sink path.
        for (index, source_line) in code.lines().enumerate() {
            let line = index + 1;
            let trimmed = source_line.trim_start();
            let lower = trimmed.to_ascii_lowercase();

            if !Self::is_sink_call(&lower) {
                continue;
            }

            let call_id = format!("{file}:call:{line}");
            let sink_id = format!("{file}:sink:{line}");

            if graph.node(&call_id).is_none() {
                graph.add_node(SecurityNode {
                    id: call_id.clone(),
                    kind: SecurityNodeKind::Call,
                    label: trimmed.to_string(),
                    file: file.to_string(),
                    line,
                });
            }

            if graph.node(&sink_id).is_none() {
                graph.add_node(SecurityNode {
                    id: sink_id.clone(),
                    kind: SecurityNodeKind::Sink,
                    label: trimmed.to_string(),
                    file: file.to_string(),
                    line,
                });
            }

            graph.add_edge(SecurityEdge {
                from: call_id.clone(),
                to: sink_id,
                kind: SecurityEdgeKind::Reaches,
            });

            let Some(open) = trimmed.find('(') else {
                continue;
            };

            let arguments = trimmed[open + 1..]
                .split_once(')')
                .map(|(args, _)| args)
                .unwrap_or(&trimmed[open + 1..])
                .to_ascii_lowercase();

            let variables: Vec<String> = graph
                .nodes()
                .filter(|node| node.kind == SecurityNodeKind::Variable && node.file == file)
                .filter(|node| {
                    arguments
                        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                        .any(|token| token == node.label.to_ascii_lowercase())
                })
                .filter(|node| tainted_variables.contains(&node.label.to_ascii_lowercase()))
                .map(|node| node.id.clone())
                .collect();

            for variable_id in variables {
                graph.add_edge(SecurityEdge {
                    from: variable_id,
                    to: call_id.clone(),
                    kind: SecurityEdgeKind::FlowsTo,
                });
            }
        }

        graph
    }

    fn walk(node: Node, context: &mut BuildContext<'_>, current_function: Option<String>) {
        match node.kind() {
            "function_definition" => {
                let function_name = node
                    .child_by_field_name("name")
                    .and_then(|n| n.utf8_text(context.code.as_bytes()).ok())
                    .unwrap_or("")
                    .to_string();

                if !function_name.is_empty() {
                    let function_id = format!(
                        "{}:function:{}:{}",
                        context.file,
                        function_name,
                        node.start_position().row + 1
                    );

                    context.graph.add_node(SecurityNode {
                        id: function_id.clone(),
                        kind: SecurityNodeKind::Function,
                        label: function_name.clone(),
                        file: context.file.to_string(),
                        line: node.start_position().row + 1,
                    });

                    if let Some(parameters) = node.child_by_field_name("parameters") {
                        Self::walk_parameters(parameters, context, &function_id);
                    }

                    if let Some(body) = node.child_by_field_name("body") {
                        Self::walk(body, context, Some(function_id));
                    }
                }
            }

            "call" => {
                let call_text = node.utf8_text(context.code.as_bytes()).unwrap_or("");

                let call_id = format!("{}:call:{}", context.file, node.start_position().row + 1);

                context.graph.add_node(SecurityNode {
                    id: call_id.clone(),
                    kind: SecurityNodeKind::Call,
                    label: call_text.to_string(),
                    file: context.file.to_string(),
                    line: node.start_position().row + 1,
                });

                if let Some(function) = current_function.as_ref() {
                    context.graph.add_edge(SecurityEdge {
                        from: function.clone(),
                        to: call_id.clone(),
                        kind: SecurityEdgeKind::Calls,
                    });
                }

                let normalized = call_text.to_ascii_lowercase();

                if Self::is_source_call(&normalized) {
                    let source_id =
                        format!("{}:source:{}", context.file, node.start_position().row + 1);

                    context.graph.add_node(SecurityNode {
                        id: source_id.clone(),
                        kind: SecurityNodeKind::Source,
                        label: call_text.to_string(),
                        file: context.file.to_string(),
                        line: node.start_position().row + 1,
                    });

                    context.graph.add_edge(SecurityEdge {
                        from: source_id,
                        to: call_id.clone(),
                        kind: SecurityEdgeKind::FlowsTo,
                    });
                }

                if SecurityDatabase::new().is_sanitizer(&normalized) {
                    let sanitizer_id = format!(
                        "{}:sanitizer:{}",
                        context.file,
                        node.start_position().row + 1
                    );

                    context.graph.add_node(SecurityNode {
                        id: sanitizer_id.clone(),
                        kind: SecurityNodeKind::Sanitizer,
                        label: call_text.to_string(),
                        file: context.file.to_string(),
                        line: node.start_position().row + 1,
                    });

                    context.graph.add_edge(SecurityEdge {
                        from: call_id.clone(),
                        to: sanitizer_id,
                        kind: SecurityEdgeKind::Sanitizes,
                    });
                }

                if Self::is_sink_call(&normalized) {
                    let sink_id =
                        format!("{}:sink:{}", context.file, node.start_position().row + 1);

                    context.graph.add_node(SecurityNode {
                        id: sink_id.clone(),
                        kind: SecurityNodeKind::Sink,
                        label: call_text.to_string(),
                        file: context.file.to_string(),
                        line: node.start_position().row + 1,
                    });

                    context.graph.add_edge(SecurityEdge {
                        from: call_id.clone(),
                        to: sink_id,
                        kind: SecurityEdgeKind::Reaches,
                    });
                }
            }

            "return_statement" => {
                let line = node.start_position().row + 1;
                let text = node.utf8_text(context.code.as_bytes()).unwrap_or("");
                let return_id = format!("{}:return:{}", context.file, line);

                if context.graph.node(&return_id).is_none() {
                    context.graph.add_node(SecurityNode {
                        id: return_id,
                        kind: SecurityNodeKind::Return,
                        label: text.trim().to_string(),
                        file: context.file.to_string(),
                        line,
                    });
                }
            }

            "assignment" => {
                if let (Some(left), Some(right)) = (
                    node.child_by_field_name("left"),
                    node.child_by_field_name("right"),
                ) {
                    let variable = left.utf8_text(context.code.as_bytes()).unwrap_or("").trim();

                    if Self::is_identifier(variable) {
                        let variable_id = format!(
                            "{}:variable:{}:{}",
                            context.file,
                            variable,
                            node.start_position().row + 1
                        );

                        context.graph.add_node(SecurityNode {
                            id: variable_id.clone(),
                            kind: SecurityNodeKind::Variable,
                            label: variable.to_string(),
                            file: context.file.to_string(),
                            line: node.start_position().row + 1,
                        });

                        let right_text = right
                            .utf8_text(context.code.as_bytes())
                            .unwrap_or("")
                            .to_ascii_lowercase();

                        if Self::contains_source(&right_text)
                            || context.tainted_variables.iter().any(|name| {
                                right_text
                                    .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                                    .any(|token| token == name)
                            })
                        {
                            context
                                .tainted_variables
                                .insert(variable.to_ascii_lowercase());

                            let source_id = format!(
                                "{}:source:assignment:{}:{}",
                                context.file,
                                variable,
                                node.start_position().row + 1
                            );

                            context.graph.add_node(SecurityNode {
                                id: source_id.clone(),
                                kind: SecurityNodeKind::Source,
                                label: right_text.clone(),
                                file: context.file.to_string(),
                                line: node.start_position().row + 1,
                            });

                            context.graph.add_edge(SecurityEdge {
                                from: source_id,
                                to: variable_id,
                                kind: SecurityEdgeKind::FlowsTo,
                            });
                        }
                    }
                }
            }

            _ => {}
        }

        let mut cursor = node.walk();

        for child in node.children(&mut cursor) {
            if node.kind() == "function_definition" && child.kind() == "block" {
                continue;
            }

            Self::walk(child, context, current_function.clone());
        }
    }

    fn walk_parameters(node: Node, context: &mut BuildContext<'_>, function_id: &str) {
        let text = node.utf8_text(context.code.as_bytes()).unwrap_or("");

        for parameter in text.trim_matches(|c| c == '(' || c == ')').split(',') {
            let name = parameter
                .trim()
                .split('=')
                .next()
                .unwrap_or("")
                .trim()
                .trim_start_matches('*');

            if !Self::is_identifier(name) {
                continue;
            }

            let parameter_id = format!(
                "{}:parameter:{}:{}",
                context.file,
                name,
                node.start_position().row + 1
            );

            context.graph.add_node(SecurityNode {
                id: parameter_id.clone(),
                kind: SecurityNodeKind::Variable,
                label: name.to_string(),
                file: context.file.to_string(),
                line: node.start_position().row + 1,
            });

            context.graph.add_edge(SecurityEdge {
                from: function_id.to_string(),
                to: parameter_id,
                kind: SecurityEdgeKind::Defines,
            });
        }
    }

    fn connect_interprocedural_edges(graph: &mut SecurityGraph, code: &str, file: &str) {
        let functions: Vec<SecurityNode> = graph
            .nodes()
            .filter(|node| node.file == file && node.kind == SecurityNodeKind::Function)
            .cloned()
            .collect();

        let calls: Vec<SecurityNode> = graph
            .nodes()
            .filter(|node| node.file == file && node.kind == SecurityNodeKind::Call)
            .cloned()
            .collect();

        let returns: Vec<SecurityNode> = graph
            .nodes()
            .filter(|node| node.file == file && node.kind == SecurityNodeKind::Return)
            .cloned()
            .collect();

        let variables: Vec<SecurityNode> = graph
            .nodes()
            .filter(|node| node.file == file && node.kind == SecurityNodeKind::Variable)
            .cloned()
            .collect();

        // Call -> callee function and argument -> parameter.
        for call in &calls {
            let call_name = Self::call_name(&call.label);
            if call_name.is_empty() {
                continue;
            }

            let Some(function) = functions
                .iter()
                .find(|function| function.label == call_name)
            else {
                continue;
            };

            graph.add_edge(SecurityEdge {
                from: call.id.clone(),
                to: function.id.clone(),
                kind: SecurityEdgeKind::Calls,
            });

            let parameters: Vec<SecurityNode> = variables
                .iter()
                .filter(|node| node.id.contains(":parameter:") && node.line == function.line)
                .cloned()
                .collect();

            for (index, argument) in Self::call_arguments(&call.label).iter().enumerate() {
                let Some(parameter) = parameters.get(index) else {
                    continue;
                };

                graph.add_edge(SecurityEdge {
                    from: call.id.clone(),
                    to: parameter.id.clone(),
                    kind: SecurityEdgeKind::FlowsTo,
                });

                if let Some(argument_variable) =
                    Self::latest_variable_before(&variables, argument, call.line)
                {
                    graph.add_edge(SecurityEdge {
                        from: argument_variable.id.clone(),
                        to: call.id.clone(),
                        kind: SecurityEdgeKind::FlowsTo,
                    });
                }
            }
        }

        // Propagate taint through local assignments, including parameter -> local variable.
        for variable in &variables {
            if variable.id.contains(":parameter:") {
                continue;
            }

            let Some(function) = Self::enclosing_function(&functions, code, variable.line) else {
                continue;
            };

            let Some(line_text) = Self::line_at(code, variable.line) else {
                continue;
            };

            let Some(rhs) = line_text.split_once('=').map(|(_, rhs)| rhs.trim()) else {
                continue;
            };

            for candidate in &variables {
                if candidate.id == variable.id
                    || !Self::variable_in_function(candidate, function, code)
                {
                    continue;
                }

                if rhs.contains(&candidate.label) {
                    graph.add_edge(SecurityEdge {
                        from: candidate.id.clone(),
                        to: variable.id.clone(),
                        kind: SecurityEdgeKind::FlowsTo,
                    });
                }
            }
        }

        // Connect values to return statements inside the same function.
        for return_node in &returns {
            let Some(function) = Self::enclosing_function(&functions, code, return_node.line)
            else {
                continue;
            };

            let Some(line_text) = Self::line_at(code, return_node.line) else {
                continue;
            };

            let expression = line_text.trim().strip_prefix("return").unwrap_or("").trim();

            for variable in &variables {
                if !Self::variable_in_function(variable, function, code) {
                    continue;
                }

                if expression.contains(&variable.label) {
                    graph.add_edge(SecurityEdge {
                        from: variable.id.clone(),
                        to: return_node.id.clone(),
                        kind: SecurityEdgeKind::FlowsTo,
                    });
                }
            }
        }

        // Connect function returns to caller assignment variables.
        for call in &calls {
            let call_name = Self::call_name(&call.label);
            if call_name.is_empty() {
                continue;
            }

            let Some(function) = functions
                .iter()
                .find(|function| function.label == call_name)
            else {
                continue;
            };

            let Some(caller_variable) = variables.iter().find(|variable| {
                !variable.id.contains(":parameter:")
                    && variable.line == call.line
                    && Self::line_at(code, variable.line)
                        .map(|line| {
                            line.split_once('=')
                                .map(|(_, rhs)| rhs.contains(&format!("{}(", call_name)))
                                .unwrap_or(false)
                        })
                        .unwrap_or(false)
            }) else {
                continue;
            };

            for return_node in &returns {
                let Some(return_function) =
                    Self::enclosing_function(&functions, code, return_node.line)
                else {
                    continue;
                };

                if return_function.id == function.id {
                    graph.add_edge(SecurityEdge {
                        from: return_node.id.clone(),
                        to: caller_variable.id.clone(),
                        kind: SecurityEdgeKind::Returns,
                    });
                }
            }
        }
    }

    fn call_name(text: &str) -> String {
        text.split('(')
            .next()
            .unwrap_or("")
            .trim()
            .rsplit('.')
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    }

    fn call_arguments(text: &str) -> Vec<String> {
        let Some((_, rest)) = text.split_once('(') else {
            return Vec::new();
        };

        rest.split_once(')')
            .map(|(args, _)| args)
            .unwrap_or(rest)
            .split(',')
            .map(str::trim)
            .filter(|arg| !arg.is_empty())
            .map(|arg| {
                arg.trim_matches(|c: char| c == '(' || c == ')' || c == ' ')
                    .to_string()
            })
            .collect()
    }

    fn latest_variable_before<'a>(
        variables: &'a [SecurityNode],
        argument: &str,
        line: usize,
    ) -> Option<&'a SecurityNode> {
        let argument = argument
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
            .next()
            .unwrap_or("");

        if !Self::is_identifier(argument) {
            return None;
        }

        variables
            .iter()
            .filter(|node| node.line <= line && node.label == argument)
            .max_by_key(|node| node.line)
    }

    fn enclosing_function<'a>(
        functions: &'a [SecurityNode],
        code: &str,
        line: usize,
    ) -> Option<&'a SecurityNode> {
        functions
            .iter()
            .filter(|function| {
                function.line < line
                    && Self::line_indent(code, line) > Self::line_indent(code, function.line)
            })
            .max_by_key(|function| function.line)
    }

    fn variable_in_function(variable: &SecurityNode, function: &SecurityNode, code: &str) -> bool {
        if variable.id.contains(":parameter:") {
            return variable.line == function.line;
        }

        Self::enclosing_function(std::slice::from_ref(function), code, variable.line)
            .map(|owner| owner.id == function.id)
            .unwrap_or(false)
    }

    fn line_at(code: &str, line: usize) -> Option<&str> {
        code.lines().nth(line.saturating_sub(1))
    }

    fn line_indent(code: &str, line: usize) -> usize {
        Self::line_at(code, line)
            .map(|text| text.chars().take_while(|c| *c == ' ' || *c == '\t').count())
            .unwrap_or(0)
    }

    fn is_source_call(text: &str) -> bool {
        SecurityDatabase::new().is_source(text)
    }

    fn contains_source(text: &str) -> bool {
        let database = SecurityDatabase::new();

        if database.is_source(text) {
            return true;
        }

        text.split(|c: char| !c.is_ascii_alphanumeric() && c != '_' && c != '.')
            .any(|token| database.is_source(token))
    }

    fn is_sink_call(text: &str) -> bool {
        SecurityDatabase::new().is_sink(text)
    }

    fn is_identifier(text: &str) -> bool {
        !text.is_empty()
            && text.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
            && text
                .chars()
                .next()
                .map(|c| c.is_ascii_alphabetic() || c == '_')
                .unwrap_or(false)
    }
}

struct BuildContext<'a> {
    code: &'a str,
    file: &'a str,
    graph: &'a mut SecurityGraph,
    tainted_variables: HashSet<String>,
}
