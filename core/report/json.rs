#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use crate::rules::finding::Finding;

use serde::Serialize;

#[derive(Serialize)]
pub struct JsonReport {
    pub findings: Vec<Finding>,
}

pub fn generate(findings: &[Finding]) -> String {
    let report = JsonReport {
        findings: findings.to_vec(),
    };

    serde_json::to_string_pretty(&report).unwrap_or_else(|_| "{}".to_string())
}
