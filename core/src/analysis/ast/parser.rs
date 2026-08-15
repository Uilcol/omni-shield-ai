use super::ASTNode;

pub struct ASTParser;

impl ASTParser {
    pub fn parse(code: &str) -> Vec<ASTNode> {
        let mut nodes = Vec::new();

        for line in code.lines() {
            let l = line.trim();

            if l.contains("=") {
                let parts: Vec<&str> = l.split('=').collect();
                if parts.len() == 2 {
                    nodes.push(ASTNode::Assign {
                        var: parts[0].trim().to_string(),
                        value: parts[1].trim().to_string(),
                    });
                    continue;
                }
            }

            if l.contains("(") && l.contains(")") {
                let name = l.split("(").next().unwrap_or("").to_string();
                nodes.push(ASTNode::Call { name });
                continue;
            }

            nodes.push(ASTNode::Unknown(l.to_string()));
        }

        nodes
    }
}
