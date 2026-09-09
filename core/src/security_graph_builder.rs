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

    fn is_source_call(text: &str) -> bool {
        text.starts_with("input(")
            || text.contains("request.get(")
            || text.contains("request.post(")
            || text.contains("sys.argv")
    }

    fn contains_source(text: &str) -> bool {
        text.contains("input(")
            || text.contains("request.get")
            || text.contains("request.post")
            || text.contains("sys.argv")
            || text.contains("user_input")
            || text.contains("params")
    }

    fn is_sink_call(text: &str) -> bool {
        text.starts_with("eval(")
            || text.starts_with("exec(")
            || text.starts_with("os.system(")
            || text.starts_with("subprocess.call(")
            || text.starts_with("cursor.execute(")
            || text.contains(".execute(")
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
