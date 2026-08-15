#![allow(dead_code)]

use crate::explanation::reason_graph::ReasonGraph;

pub struct ExplanationFormatter;

impl ExplanationFormatter {
    pub fn format(graph: &ReasonGraph) -> String {
        let mut output = String::new();

        for (id, node) in &graph.nodes {
            output.push_str(&format!("Node {}: {}\n", id, node.description));

            for child in &node.links {
                output.push_str(&format!("  -> {}\n", child));
            }
        }

        output
    }
}
