use crate::deep::taint_real::{TaintEngineReal, TaintResult};
use crate::deep::z3_real::{
    PathConstraint,
    Z3FalsePositiveKiller,
};

#[derive(Debug, Clone)]
pub struct HybridFinding {
    pub source: String,
    pub sink: String,
    pub exploitable: bool,
    pub validated_by_z3: bool,
}

pub struct HybridEngine;

impl HybridEngine {
    pub fn analyze(code: &str) -> Vec<HybridFinding> {
        let taint_results: Vec<TaintResult> =
            TaintEngineReal::analyze(code);

        let mut findings = Vec::new();

        for finding in taint_results {
            let constraints = vec![
                PathConstraint {
                    variable: "input".to_string(),
                    op: "!=".to_string(),
                    value: 0,
                },
                PathConstraint {
                    variable: "input".to_string(),
                    op: ">".to_string(),
                    value: -1,
                },
            ];

            let z3 =
                Z3FalsePositiveKiller::validate(constraints);

            findings.push(HybridFinding {
                source: finding.source,
                sink: finding.sink,
                exploitable: z3.exploitable,
                validated_by_z3: z3.satisfiable,
            });
        }

        findings
    }
}
