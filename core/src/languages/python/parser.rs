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
