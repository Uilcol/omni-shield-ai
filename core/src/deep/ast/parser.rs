use super::nodes::ASTNode;

pub struct ASTParser;

impl ASTParser {
    pub fn parse(code: &str) -> Vec<ASTNode> {
        let mut nodes = Vec::new();

        for line in code.lines() {
            let line = line.trim();

            if line.contains("if") {
                nodes.push(ASTNode::If {
                    condition: line.to_string(),
                });
            } else if line.contains("=") {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 {
                    nodes.push(ASTNode::Assign {
                        var: parts[0].trim().to_string(),
                        value: parts[1].trim().to_string(),
                    });
                }
            } else if line.contains("input") {
                nodes.push(ASTNode::Input {
                    var: line.to_string(),
                });
            } else {
                nodes.push(ASTNode::Unknown(line.to_string()));
            }
        }

        nodes
    }
}
