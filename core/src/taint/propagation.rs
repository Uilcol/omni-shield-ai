#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::{HashMap, HashSet};

pub struct TaintPropagation {
    graph: HashMap<String, Vec<String>>,
}

impl TaintPropagation {
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.graph.entry(from).or_insert(Vec::new()).push(to);
    }

    pub fn propagate(&self, source: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut stack = vec![source.to_string()];

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }

            visited.insert(node.clone());

            if let Some(neighbors) = self.graph.get(&node) {
pub fn run() {
pub fn run() {
                        for n in neighbors {
                    stack.push(n.clone());
}


            }
        }

        visited
    }
}
