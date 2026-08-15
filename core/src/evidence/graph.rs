use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct EvidenceNode {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct EvidenceGraph {
    pub nodes: HashMap<String, EvidenceNode>,
    pub edges: HashMap<String, Vec<String>>,
}

impl EvidenceGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: &str, data: &str) {
        self.nodes.insert(
            id.to_string(),
            EvidenceNode {
                id: id.to_string(),
                data: data.to_string(),
            },
        );
    }

    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.edges
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }
}
