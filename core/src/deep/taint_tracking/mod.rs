use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TaintState {
    pub tainted: HashSet<String>,
    pub flows: HashMap<String, Vec<String>>,
}

pub struct TaintTrackingEngine;

impl TaintTrackingEngine {
    pub fn analyze(code: &str) -> TaintState {
        let mut tainted = HashSet::new();
        let mut flows = HashMap::new();

        for line in code.lines() {
            let l = line.trim();

            if l.contains("input(")
                || l.contains("read_line")
                || l.contains("request.")
            {
                if let Some(var) = l.split('=').next() {
                    tainted.insert(
                        var.trim().to_string(),
                    );
                }
            }

            if l.contains('=') {
                let parts: Vec<&str> =
                    l.split('=').collect();

                if parts.len() == 2 {
                    let left =
                        parts[0].trim();

                    let right =
                        parts[1].trim();

                    for t in tainted.clone() {
                        if right.contains(&t) {
                            tainted.insert(
                                left.to_string(),
                            );

                            flows
                                .entry(t.clone())
                                .or_insert(Vec::new())
                                .push(
                                    left.to_string(),
                                );
                        }
                    }
                }
            }
        }

        TaintState {
            tainted,
            flows,
        }
    }
}
