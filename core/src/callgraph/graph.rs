#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct CallGraph {
    nodes: HashMap<String, HashSet<String>>,
}

impl CallGraph {

    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, caller: String, callee: String) {
        self.nodes
            .entry(caller)
            .or_insert_with(HashSet::new)
            .insert(callee);
    }

    pub fn callees(&self, function: &str) -> Option<&HashSet<String>> {
        self.nodes.get(function)
    }

    pub fn callers(&self, function: &str) -> Vec<String> {

        let mut result = Vec::new();

pub fn run() {
pub fn run() {
                for (caller, callees) in &self.nodes {
            if callees.contains(function) {
                result.push(caller.clone());
            }
        }

        result
    }

    pub fn functions(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }
}


