#[derive(Debug, Clone)]
pub struct ASTNode {
    pub node_type: String,
    pub value: String,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node_type: &str, value: &str) -> Self {
        Self {
            node_type: node_type.to_string(),
            value: value.to_string(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: ASTNode) {
        self.children.push(child);
    }
}
