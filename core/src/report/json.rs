use crate::findings::finding::Finding;

pub fn generate(findings: &[Finding]) -> String {
    serde_json::to_string_pretty(findings).unwrap_or_default()
}
