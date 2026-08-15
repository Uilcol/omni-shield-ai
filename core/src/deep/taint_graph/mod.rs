
#[derive(Debug, Clone)]
pub struct TaintNode {
    pub id: usize,
    pub name: String,
    pub tainted: bool,
}

#[derive(Debug, Clone)]
pub struct TaintEdge {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug, Clone)]
pub struct TaintGraph {
    pub nodes: Vec<TaintNode>,
    pub edges: Vec<TaintEdge>,
}

pub struct TaintGraphEngine;

impl TaintGraphEngine {
    pub fn build(code: &str) -> TaintGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        let mut last_id: Option<usize> = None;

        for (i, line) in code.lines().enumerate() {
            let tainted =
                line.contains("input")
                || line.contains("read")
                || line.contains("request");

            nodes.push(TaintNode {
                id: i,
                name: line.trim().to_string(),
                tainted,
            });

            if let Some(prev) = last_id {
                edges.push(TaintEdge {
                    from: prev,
                    to: i,
                });
            }

            last_id = Some(i);
        }

        TaintGraph { nodes, edges }
    }

    pub fn propagate(
        graph: &mut TaintGraph,
    ) {
        let mut changed = true;

        while changed {
            changed = false;

            let current = graph.nodes.clone();

            for edge in &graph.edges {
                let src = current
                    .iter()
                    .find(|n| n.id == edge.from);

                let dst = graph
                    .nodes
                    .iter_mut()
                    .find(|n| n.id == edge.to);

                if let (Some(s), Some(d)) = (src, dst) {
                    if s.tainted && !d.tainted {
                        d.tainted = true;
                        changed = true;
                    }
                }
            }
        }
    }

    pub fn sinks(
        graph: &TaintGraph,
    ) -> Vec<String> {
        graph.nodes
            .iter()
            .filter(|n| {
                n.tainted &&
                (
                    n.name.contains("exec")
                    || n.name.contains("eval")
                    || n.name.contains("query")
                )
            })
            .map(|n| n.name.clone())
            .collect()
    }
}
