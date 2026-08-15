
#[derive(Debug, Clone)]
pub struct CFGNode {
    pub id: usize,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct CFGEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct RealCFG {
    pub entry: usize,
    pub nodes: Vec<CFGNode>,
    pub edges: Vec<CFGEdge>,
}

pub struct CFGRealBuilder;

impl CFGRealBuilder {
    pub fn build(code: &str) -> RealCFG {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let mut prev: Option<usize> = None;

        for (idx, line) in code.lines().enumerate() {
            let id = idx;

            nodes.push(CFGNode {
                id,
                code: line.trim().to_string(),
            });

            if let Some(p) = prev {
                edges.push(CFGEdge {
                    from: p,
                    to: id,
                });
            }

            prev = Some(id);
        }

        RealCFG {
            entry: 0,
            nodes,
            edges,
        }
    }

    pub fn successors(
        cfg: &RealCFG,
        node: usize,
    ) -> Vec<usize> {
        cfg.edges
            .iter()
            .filter(|e| e.from == node)
            .map(|e| e.to)
            .collect()
    }

    pub fn predecessors(
        cfg: &RealCFG,
        node: usize,
    ) -> Vec<usize> {
        cfg.edges
            .iter()
            .filter(|e| e.to == node)
            .map(|e| e.from)
            .collect()
    }
}
