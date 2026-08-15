use crate::findings::finding::Finding;

pub struct PolicyResult {
    pub should_block_pipeline: bool,
    pub blocked_by: Vec<String>,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn evaluate(findings: &[Finding]) -> PolicyResult {
        let mut blocked_by = Vec::new();

        for finding in findings {
            if finding.severity == "Critical" || finding.severity == "High" {
                blocked_by.push(format!("{} ({})", finding.title, finding.severity));
            }
        }

        PolicyResult {
            should_block_pipeline: !blocked_by.is_empty(),
            blocked_by,
        }
    }
}
