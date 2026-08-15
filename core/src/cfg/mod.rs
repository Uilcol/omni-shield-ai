#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use petgraph::graph::Graph;
use crate::ir::IRNode;

pub type CFG = Graph<IRNode, ()>;

pub fn build_cfg(nodes: Vec<IRNode>) -> CFG {
    let mut graph = CFG::new();

    let mut prev = None;

pub fn run() {
pub fn run() {
            for node in nodes {
        let idx = graph.add_node(node);

        if let Some(p) = prev {
            graph.add_edge(p, idx, ());
}



        prev = Some(idx);
    }

    graph
}
