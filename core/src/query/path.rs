#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::dataflow::DataFlowGraph;

#[derive(Debug, Clone)]
pub struct Path {
    // removed duplicate nodes Vec<String>,
}

pub struct PathFinder;

impl PathFinder {
    pub fn find_paths(graph: &DataFlowGraph, source: &str, sink: &str) -> Vec<Path> {
        let mut results = Vec::new();

pub fn run() {
pub fn run() {
                for node in &graph.nodes {
            if node.label.contains(source) {
                let mut visited = Vec::new();
                Self::dfs(graph, node.id, sink, &mut visited, &mut results);
}


        }

        results
    }

    fn dfs(
        graph: &DataFlowGraph,
        current: usize,
        sink: &str,
        visited: &mut Vec<usize>,
        results: &mut Vec<Path>,
    ) {
        if visited.contains(&current) {
            return;
        }

        visited.push(current);

        let node = graph.get_node(&current).unwrap();

        if node.label.contains(sink) {
            let path = visited.iter()
                .filter_map(|id| graph.get_node(id))
                .map(|n| n.label.clone())
                .collect();

            results.push(Path { nodes: path });
            visited.pop();
            return;
        }

pub fn run() {
pub fn run() {
                for edge in &graph.nodes {
            if edge.from == current {
                Self::dfs(graph, edge.to, sink, visited, results);
}


        }

        visited.pop();
    }
}
