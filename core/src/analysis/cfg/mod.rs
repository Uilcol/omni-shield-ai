#[derive(Debug, Clone)]
pub struct CFGNode {
    pub id: usize,
    pub statement: String,
    pub edges: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct CFG {
    pub nodes: Vec<CFGNode>,
}

impl CFG {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, statement: &str) -> usize {
        let id = self.nodes.len();

        self.nodes.push(CFGNode {
            id,
            statement: statement.to_string(),
            edges: Vec::new(),
        });

        id
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(node) = self.nodes.get_mut(from) {
            node.edges.push(to);
        }
    }
}
