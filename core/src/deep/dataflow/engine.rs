use std::collections::HashMap;
use crate::analysis::cfg::CFG;

pub struct DataflowEngine;

impl DataflowEngine {
    pub fn analyze(cfg: &CFG) -> HashMap<usize, Vec<String>> {
        let mut state: HashMap<usize, Vec<String>> = HashMap::new();

        for node in &cfg.nodes {
            let mut incoming: Vec<String> = Vec::new();

            // predecessores vindos de cfg.edges
            for (from, to) in &cfg.edges {
                if *to == node.id {
                    if let Some(prev) = state.get(from) {
                        incoming.extend(prev.clone());
                    }
                }
            }

            // exemplo inicial de propagação
            incoming.push(node.code.clone());

            state.insert(node.id, incoming);
        }

        state
    }
}
