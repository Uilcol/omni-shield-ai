use crate::analysis::cfg::CFG;
use std::collections::HashMap;

pub struct SSAEngine;

impl SSAEngine {
    pub fn transform(cfg: &CFG) -> HashMap<String, usize> {
        let mut versions = HashMap::new();

        for node in &cfg.nodes {
            let code = &node.code;

            if code.contains("=") {
                let parts: Vec<&str> = code.split('=').collect();
                let var = parts[0].trim();

                let count = versions.entry(var.to_string()).or_insert(0);
                *count += 1;
            }
        }

        versions
    }
}
