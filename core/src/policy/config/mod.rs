use std::fs;

#[derive(Debug, Clone)]
pub struct PolicyConfig {
    pub block_severity: Vec<String>,
    pub ignored_paths: Vec<String>,
}

pub struct PolicyConfigLoader;

impl PolicyConfigLoader {
    pub fn load(path: &str) -> PolicyConfig {
        let content = fs::read_to_string(path).unwrap_or_default();

        let mut block_severity = Vec::new();
        let mut ignored_paths = Vec::new();

        let mut current_section = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("block_severity:") {
                current_section = "severity".to_string();
                continue;
            }

            if trimmed.starts_with("ignored_paths:") {
                current_section = "paths".to_string();
                continue;
            }

            if trimmed.starts_with("- ") {
                let value = trimmed.replace("- ", "");

                match current_section.as_str() {
                    "severity" => block_severity.push(value),
                    "paths" => ignored_paths.push(value),
                    _ => {}
                }
            }
        }

        if block_severity.is_empty() {
            block_severity.push("Critical".to_string());
            block_severity.push("High".to_string());
        }

        PolicyConfig {
            block_severity,
            ignored_paths,
        }
    }
}
