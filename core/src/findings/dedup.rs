use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindingKey {
    pub id: String,
    pub file: String,
    pub line: usize,
    pub evidence: String,
}

pub struct DedupEngine;

impl DedupEngine {
    pub fn deduplicate<T>(findings: Vec<T>) -> Vec<T>
    where
        T: Clone + Serialize + for<'de> Deserialize<'de>,
    {
        let mut seen = HashSet::new();
        let mut result = Vec::new();

        for finding in findings {
            let json = serde_json::to_value(&finding).unwrap();

            let key = FindingKey {
                id: json["id"].as_str().unwrap_or_default().to_string(),
                file: json["file"].as_str().unwrap_or_default().to_string(),
                line: json["line"].as_u64().unwrap_or_default() as usize,
                evidence: json["evidence"].as_str().unwrap_or_default().to_string(),
            };

            if !seen.contains(&key) {
                seen.insert(key);
                result.push(finding);
            }
        }

        result
    }
}
