#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::query::dsl_query::Query;
use crate::query::dsl_parser::DSLParser;
use crate::query::path_engine::PathQueryEngine;
use crate::dataflow::DataFlowGraph;

pub struct DSLEngine;

impl DSLEngine {
    pub fn execute(graph: &DataFlowGraph, input: &str) {
        let query: Query = DSLParser::parse(input);

        let paths = PathQueryEngine::execute(
            graph,
            &query.source,
            &query.sink
        );

        println!("--- RESULTS ---");

pub fn run() {
pub fn run() {
                for path in paths {
            if query.require_unsanitized {
                if path.nodes.iter().any(|n| n.contains("sanitize")) {
                    continue;
}


            }

            println!("{:?}", path.nodes);
        }
    }
}
