use crate::security_graph::SecurityPath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: String,
    pub cwe: String,
    pub owasp: String,
    pub confidence: f64,
    pub file: String,
    pub line: usize,
    pub evidence: String,
    pub recommendation: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_path: Option<SecurityPath>,
}

impl Finding {
    pub fn new(
        id: &str,
        title: &str,
        severity: &str,
        cwe: &str,
        owasp: &str,
        confidence: f64,
        file: &str,
        line: usize,
        evidence: &str,
        recommendation: &str,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            severity: severity.to_string(),
            cwe: cwe.to_string(),
            owasp: owasp.to_string(),
            confidence,
            file: file.to_string(),
            line,
            evidence: evidence.to_string(),
            recommendation: recommendation.to_string(),
            security_path: None,
        }
    }
}
