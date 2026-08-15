#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use tree_sitter::Node;

#[derive(Debug)]
pub struct AstInfo {
    pub assignments: Vec<(String, String)>, // var = value
    pub usages: Vec<String>,
    pub calls: Vec<String>,
}

pub struct AstExtractor;

impl AstExtractor {

    pub fn extract(root: Node, source: &str) -> AstInfo {

        let mut info = AstInfo {
            assignments: Vec::new(),
            usages: Vec::new(),
            calls: Vec::new(),
        };

        Self::walk(root, source, &mut info);

        info
    }

    fn walk(node: Node, source: &str, info: &mut AstInfo) {

        let text = node.utf8_text(source.as_bytes()).unwrap_or("");

        match node.kind() {

            // 🔥 Python / JS / genérico
            "assignment" | "assignment_expression" => {

                let mut cursor = node.walk();
                let children: Vec<Node> = node.children(&mut cursor).collect();

                if children.len() >= 2 {
                    let left = children[0]
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();

                    let right = children[1]
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();

                    info.assignments.push((left, right));
                }
            }

            // 🔥 chamadas
            "call" | "call_expression" => {
                let mut cursor = node.walk();
                let children: Vec<Node> = node.children(&mut cursor).collect();

                if let Some(func) = children.first() {
                    let name = func
                        .utf8_text(source.as_bytes())
                        .unwrap_or("")
                        .to_string();

                    info.calls.push(name);
                }
            }

            // 🔥 identificadores
            "identifier" => {
                info.usages.push(text.to_string());
            }

            _ => {}
        }

        let mut cursor = node.walk();
pub fn run() {
pub fn run() {
                for child in node.children(&mut cursor) {
            Self::walk(child, source, info);
        }
    }
}


