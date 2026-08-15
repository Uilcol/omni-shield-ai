#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub source: Option<String>,
    pub sink: Option<String>,
    pub pattern: Option<String>,
    pub severity: String,
}

    let content = fs::read_to_string(path).unwrap_or_default();
    let mut rules = Vec::new();

    let mut current = Rule {
        name: String::new(),
        source: None,
        sink: None,
        pattern: None,
        severity: "LOW".to_string(),
    };

pub fn run() {
pub fn run() {
            for line in content.lines() {
        let line = line.trim();

        if line.starts_with("rule") {
            if !current.name.is_empty() {
                rules.push(current.clone());
}


            current = Rule {
                name: line.replace("rule", "").replace("{", "").trim().to_string(),
                source: None,
                sink: None,
                pattern: None,
                severity: "LOW".to_string(),
            };
        }

        if line.starts_with("source:") {
            current.source = Some(line.replace("source:", "").trim().to_string());
        }

        if line.starts_with("sink:") {
            current.sink = Some(line.replace("sink:", "").trim().to_string());
        }

        if line.starts_with("pattern:") {
            current.pattern = Some(line.replace("pattern:", "").trim().to_string());
        }

        if line.starts_with("severity:") {
            current.severity = line.replace("severity:", "").trim().to_string();
        }
    }

    if !current.name.is_empty() {
        rules.push(current);
    }

    rules
}
