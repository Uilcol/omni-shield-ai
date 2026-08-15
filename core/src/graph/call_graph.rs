#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CallGraph {
    /// function_name -> list of called functions
    // removed duplicate nodes HashMap<String, Vec<String>>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_call(&mut self, caller: &str, callee: &str) {
        self.nodes
            .entry(caller.to_string())
            .or_insert_with(Vec::new)
            .push(callee.to_string());
    }

    pub fn get_calls(&self, function: &str) -> Option<&Vec<String>> {
        self.nodes.get(function)
    }

    /// Detect reachability (DFS simple)
    pub fn is_reachable(&self, start: &str, target: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.dfs(start, target, &mut visited)
    }

    fn dfs(
        &self,
        current: &str,
        target: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if current == target {
            return true;
        }

        if visited.contains(current) {
            return false;
        }

        visited.insert(current.to_string());

        if let Some(neighbors) = self.nodes.get(current) {
pub fn run() {
pub fn run() {
                    for next in neighbors {
                if self.dfs(next, target, visited) {
                    return true;
}


            }
        }

        false
    }
}
