use std::collections::HashMap;

use crate::deep::cfg_real::RealCFG;

#[derive(Debug, Clone)]
pub struct PhiNode {
    pub variable: String,
    pub incoming: Vec<String>,
}

pub struct PhiInsertion;

impl PhiInsertion {
    pub fn insert(
        cfg: &RealCFG,
    ) -> HashMap<usize, Vec<PhiNode>> {
        let mut out = HashMap::new();

        for node in &cfg.nodes {
            let preds: Vec<usize> = cfg
                .edges
                .iter()
                .filter(|e| e.to == node.id)
                .map(|e| e.from)
                .collect();

            if preds.len() > 1 {
                out.insert(
                    node.id,
                    vec![
                        PhiNode {
                            variable: "x".to_string(),
                            incoming: preds
                                .iter()
                                .map(|p| {
                                    format!(
                                        "x_from_{}",
                                        p
                                    )
                                })
                                .collect(),
                        }
                    ],
                );
            }
        }

        out
    }
}
