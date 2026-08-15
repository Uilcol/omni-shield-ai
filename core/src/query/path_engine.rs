#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::query::path_pruner::PathPruner;
use crate::dataflow::DataFlowGraph;

pub struct PathQueryEngine;

impl PathQueryEngine {
    pub fn execute(graph: &DataFlowGraph, source: &str, sink: &str) -> Vec<Path> {
        let paths = PathFinder::find_paths(graph, source, sink);
        PathPruner::prune(paths)
    }
}
