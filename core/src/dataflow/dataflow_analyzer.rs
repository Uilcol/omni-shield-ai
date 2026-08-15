#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;

use super::flow_graph::FlowGraph;

pub struct DataflowAnalyzer;

impl DataflowAnalyzer {

    pub fn new() -> Self {

        Self

    }


        let mut graph = FlowGraph::new();

        let content = fs::read_to_string(file_path).unwrap_or_default();

        let mut previous = None;

pub fn run() {
pub fn run() {
                for line in content.lines() {

            let node_id = graph.add_node(line.trim().to_string());

            if let Some(prev) = previous {

                graph.add_edge(prev, node_id);

            }

            previous = Some(node_id);

        }

        graph

    }
}


