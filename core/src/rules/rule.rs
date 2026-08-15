#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::severity::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub pattern: String,
    pub message: String,
    pub severity: Severity,
    pub languages: Vec<String>,
}

impl Rule {
    pub fn new(
        id: &str,
        title: &str,
        description: &str,
        pattern: &str,
        message: &str,
        severity: Severity,
        languages: Vec<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            pattern: pattern.to_string(),
            message: message.to_string(),
            severity,
            languages,
        }
    }
}
