#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct DataFlowNode {
    pub id: usize,
    pub label: String,
}

impl DataFlowNode {
    pub fn new(id: usize, label: String) -> Self {
        Self { id, label }
    }
}

#[derive(Debug, Clone)]
pub struct DataFlowEdge {
    pub from: usize,
    pub to: usize,
}

impl DataFlowEdge {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }
}

#[derive(Debug, Clone)]
pub struct DataFlowGraph {
    // removed duplicate nodes Vec<DataFlowNode>,
    // removed duplicate nodes Vec<DataFlowEdge>,
}

impl DataFlowGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            nodes: Vec::new(),
        }
    }
}
impl DataFlowGraph {
    pub fn get_node(&self, id: &usize) -> Option<&DataFlowNode> {
        self.nodes.iter().find(|n| &n.id == id)
    }
}
}
