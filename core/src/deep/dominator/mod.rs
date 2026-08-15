use std::collections::{HashMap, HashSet};

use crate::deep::cfg_real::RealCFG;

pub struct DominatorTree;

impl DominatorTree {
    pub fn compute(
        cfg: &RealCFG,
    ) -> HashMap<usize, HashSet<usize>> {
        let mut dom: HashMap<
            usize,
            HashSet<usize>,
        > = HashMap::new();

        let all_nodes: HashSet<usize> =
            cfg.nodes.iter().map(|n| n.id).collect();

        for node in &cfg.nodes {
            if node.id == cfg.entry {
                dom.insert(
                    node.id,
                    [node.id].iter().cloned().collect(),
                );
            } else {
                dom.insert(node.id, all_nodes.clone());
            }
        }

        let mut changed = true;

        while changed {
            changed = false;

            for node in &cfg.nodes {
                if node.id == cfg.entry {
                    continue;
                }

                let preds: Vec<usize> = cfg
                    .edges
                    .iter()
                    .filter(|e| e.to == node.id)
                    .map(|e| e.from)
                    .collect();

                if preds.is_empty() {
                    continue;
                }

                let mut new_dom =
                    all_nodes.clone();

                for p in preds {
                    if let Some(pdom) = dom.get(&p) {
                        new_dom = new_dom
                            .intersection(pdom)
                            .cloned()
                            .collect();
                    }
                }

                new_dom.insert(node.id);

                if dom.get(&node.id) != Some(&new_dom) {
                    dom.insert(node.id, new_dom);
                    changed = true;
                }
            }
        }

        dom
    }
}
