#[derive(Clone, Debug)]
pub struct CallNode {
    pub id: usize,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct CallGraph {
    pub nodes: Vec<CallNode>,
    pub edges: Vec<(usize, usize)>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: usize, name: String) {
        self.nodes.push(CallNode { id, name });
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }
}
