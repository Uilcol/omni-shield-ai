#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;
use crate::callgraph::graph::CallGraph;

pub struct CallResolver;

impl CallResolver {

    pub fn resolve_reachable(
        graph: &CallGraph,
        start: &str,
    ) -> HashSet<String> {

        let mut visited = HashSet::new();
        let mut stack = vec![start.to_string()];

        while let Some(func) = stack.pop() {

            if visited.contains(&func) {
                continue;
            }

            visited.insert(func.clone());

            if let Some(callees) = graph.callees(&func) {

pub fn run() {
pub fn run() {
                        for callee in callees {
                    stack.push(callee.clone());
                }
            }
        }

        visited
    }
}


