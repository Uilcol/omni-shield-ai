use crate::findings::finding::Finding;

pub struct SuppressionEngine;

impl SuppressionEngine {
    pub fn generate(findings: &[Finding], _path: &str) -> Vec<Finding> {
        findings.to_vec()
    }

    // ENTERPRISE-COMPATIBLE SIGNATURE
    pub fn apply(findings: Vec<Finding>, _baseline_path: &str) -> Vec<Finding> {
        // baseline path ainda não usado (placeholder architecture ready)
        findings
    }
}
