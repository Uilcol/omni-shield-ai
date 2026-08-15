#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub mod call_graph;
pub mod interprocedural_analyzer;

pub use call_graph::{CallGraph, CallNode};
pub use interprocedural_analyzer::InterproceduralAnalyzer;
