#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct FlowNode {

    pub id: usize,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct FlowGraph {

    // removed duplicate nodes Vec<FlowNode>,
    // removed duplicate nodes Vec<(usize, usize)>,

}

impl FlowGraph {

    pub fn new() -> Self {

        Self {

            nodes: Vec::new(),
            nodes: Vec::new(),

        }

    }

    pub fn add_node(&mut self, code: String) -> usize {

        let id = self.nodes.len();

        self.nodes.push(

            FlowNode {

                id,
                code,

            }

        );

        id

    }

    pub fn add_edge(&mut self, from: usize, to: usize) {

        self.nodes.push((from, to));

    }
}
