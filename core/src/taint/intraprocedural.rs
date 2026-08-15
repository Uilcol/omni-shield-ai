#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct IntraProceduralTaint {
    tainted: HashSet<String>,
    flows: HashMap<String, String>,
}

impl IntraProceduralTaint {
    pub fn new() -> Self {
        Self {
            tainted: HashSet::new(),
            flows: HashMap::new(),
        }
    }

    pub fn mark_source(&mut self, variable: &str) {
        self.tainted.insert(variable.to_string());
    }

    pub fn propagate(&mut self, from: &str, to: &str) {
        if self.tainted.contains(from) {
            self.tainted.insert(to.to_string());
            self.flows.insert(to.to_string(), from.to_string());
        }
    }

    pub fn is_tainted(&self, var: &str) -> bool {
        self.tainted.contains(var)
    }

    pub fn trace(&self, var: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut current = var;

        while let Some(prev) = self.flows.get(current) {
            result.push(prev.clone());
            current = prev;
        }

        result
    }
}
