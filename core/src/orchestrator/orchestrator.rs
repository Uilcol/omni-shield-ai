use crate::analyzer::engine::rule_engine::RuleEngine;
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::taint::models::TaintPath;
use crate::Finding;

pub struct Orchestrator;

impl Orchestrator {
    pub fn run(_paths: Vec<TaintPath>) -> Vec<Finding> {
        vec![]
    }
}
