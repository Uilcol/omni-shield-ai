#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::dataflow::graph::{DataFlowGraph, DataFlowNode, DataFlowEdge};

pub struct DataFlowBuilder {
    graph: DataFlowGraph,
    counter: usize,
}

impl DataFlowBuilder {
    pub fn new() -> Self {
        Self {
            graph: DataFlowGraph::new(),
            counter: 0,
        }
    }

    pub fn create_variable(&mut self, name: &str) -> usize {
        let id = self.counter;
        self.counter += 1;

        let node = DataFlowNode::new(id, name.to_string());
        self.graph.nodes.push(node);

        id
    }

    pub fn create_literal(&mut self, value: &str) -> usize {
        let id = self.counter;
        self.counter += 1;

        let node = DataFlowNode::new(id, value.to_string());
        self.graph.nodes.push(node);

        id
    }

    pub fn add_assignment(&mut self, from: usize, to: usize) {
        let edge = DataFlowEdge::new(from, to);
        self.graph.nodes.push(edge);
    }

    pub fn build(self) -> DataFlowGraph {
        self.graph
    }

    pub fn build_from_code(_code: &str) -> DataFlowGraph {
        let mut builder = Self::new();

        let a = builder.create_variable("input");
        let b = builder.create_variable("processed");

        builder.add_assignment(a, b);

        builder.build()
    }
}
