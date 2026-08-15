use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ReasonNode {
    pub id: String,
    pub description: String,
    pub links: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ReasonGraph {
    pub nodes: HashMap<String, ReasonNode>,
}

impl ReasonGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, desc: &str) {
        self.nodes.insert(
            id.to_string(),
            ReasonNode {
                id: id.to_string(),
                description: desc.to_string(),
                links: Vec::new(),
            },
        );
    }

    pub fn link(&mut self, from: &str, to: &str) {
        if let Some(node) = self.nodes.get_mut(from) {
            node.links.push(to.to_string());
        }
    }
}
