#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct Node {
    pub id: usize,
    pub code: String,
}

#[derive(Debug, Clone)]
pub struct CFG {
    // removed duplicate nodes Vec<Node>,
    // removed duplicate nodes Vec<(usize, usize)>,
}

impl CFG {
    pub fn successors(&self, id: usize) -> Vec<usize> {
        self.nodes
            .iter()
            .filter_map(|(from, to)| if *from == id { Some(*to) } else { None })
            .collect()
    }
}
