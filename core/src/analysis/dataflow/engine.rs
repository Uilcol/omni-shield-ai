#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::analysis::cfg::CFG;
use crate::taint::taint_engine::TaintEngine;
use crate::taint::models::TaintPath;

pub struct DataflowEngine;

impl DataflowEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, cfg: &CFG) -> Vec<TaintPath> {
        let taint_engine = TaintEngine::new();
        taint_engine.analyze(cfg)
    }
}
