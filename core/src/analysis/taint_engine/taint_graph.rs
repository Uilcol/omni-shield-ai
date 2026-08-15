#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TaintNode {

    pub id: usize,
    pub code: String,

}

#[derive(Debug)]
pub struct TaintGraph {

    // removed duplicate nodes Vec<TaintNode>,
    // removed duplicate nodes Vec<(usize, usize)>,

}

impl TaintGraph {

    pub fn new() -> Self {

        Self {

            nodes: Vec::new(),
            nodes: Vec::new(),

        }

    }

    pub fn add_node(&mut self, code: String) -> usize {

        let id = self.nodes.len();

        self.nodes.push(

            TaintNode {

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
