#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Debug)]
pub struct GraphDelta {
    pub affected_nodes: HashSet<String>,
}

impl GraphDelta {
    pub fn new() -> Self {
        Self {
            affected_nodes: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: &str) {
        self.affected_nodes.insert(node.to_string());
    }
}
