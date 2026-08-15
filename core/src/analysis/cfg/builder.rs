use super::{CFG, Node};

pub struct CFGBuilder;

impl CFGBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build<T: std::fmt::Debug + ?Sized>(&self, input: &T) -> CFG {
        let mut nodes = Vec::new();

        let code = format!("{:?}", input);

        for (i, line) in code.lines().enumerate() {
            nodes.push(Node {
                id: i,
                code: line.to_string(),
            });
        }

        CFG {
            nodes,
            edges: Vec::new(),
            entry: 0,
        }
    }
}
