#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::finding::Finding;

pub fn generate(findings: &Vec<Finding>) -> String {
    serde_json::to_string_pretty(findings).unwrap_or_default()
}
