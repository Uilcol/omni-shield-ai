use crate::analysis::cfg::CFG;

#[derive(Debug, Clone)]
pub struct DataFlowEdge {
    pub from: usize,
    pub to: usize,
    pub label: String,
}

pub struct DataFlowGraph;

impl DataFlowGraph {
    pub fn build(cfg: &CFG) -> Vec<DataFlowEdge> {
        let mut nodes = Vec::new();

        for i in 0..cfg.nodes.len() {
            let node = &cfg.nodes[i];

            if i + 1 < cfg.nodes.len() {
                nodes.push(DataFlowEdge {
                    from: node.id,
                    to: cfg.nodes[i + 1].id,
                    label: "flow".into(),
                });
            }

            // detectar fluxo de dados sensível
            if node.code.contains("input") {
                nodes.push(DataFlowEdge {
                    from: node.id,
                    to: node.id,
                    label: "taint-source".into(),
                });
            }
        }

        nodes
    }
}
