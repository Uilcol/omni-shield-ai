#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct FlowEdge {

    pub from: String,
    pub to: String,

}

#[derive(Default)]
pub struct FlowGraph {

    // removed duplicate nodes Vec<FlowEdge>,

}

impl FlowGraph {

    pub fn add_edge(&mut self, from: &str, to: &str) {

        self.nodes.push(
            FlowEdge {
                from: from.to_string(),
                to: to.to_string(),
            }
        );

    }
}
