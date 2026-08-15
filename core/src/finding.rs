#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::severity::Severity;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: String,
    pub rule_id: String,
    pub title: String,
    pub message: String,
    pub description: String,
    pub file: String,
    pub line: usize,
    pub severity: Severity,
    pub location: Location,
}

impl Finding {
    pub fn new(
        id: &str,
        title: &str,
        description: &str,
        file: &Path,
        line: usize,
        severity: Severity,
    ) -> Self {
        Self {
            id: id.to_string(),
            rule_id: id.to_string(),
            title: title.to_string(),
            message: description.to_string(),
            description: description.to_string(),
            file: file.to_str().unwrap_or("").to_string(),
            line,
            severity: severity.clone(),
            location: Location {
                file: file.to_path_buf(),
                line,
            },
        }
    }
}
