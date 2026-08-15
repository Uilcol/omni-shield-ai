#![allow(dead_code)]

use crate::explanation::reason_graph::ReasonGraph;

pub struct EvidenceExplanationGenerator;

impl EvidenceExplanationGenerator {
    pub fn generate(graph: &ReasonGraph) -> String {
        let mut output = String::new();

        for (id, node) in &graph.nodes {
            output.push_str(&format!("[{}] {}\n", id, node.description));
        }

        output
    }
}
