#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Rule {
    pub id: String,
    pub description: String,
    pub severity: String,
    pub pattern: String,
    pub source: Option<String>,
    pub sink: Option<String>,
    pub language: Option<String>,
    pub require_sanitized: bool,
}

#[derive(Debug)]
pub struct RuleMatch {
    pub rule_id: String,
    pub description: String,
    pub severity: String,
    pub file: String,
    pub line: usize,
}
