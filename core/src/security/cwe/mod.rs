#[derive(Debug, Clone)]
pub struct CWE {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl CWE {
    pub fn new(id: &str, name: &str, description: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct CweMapper;

impl CweMapper {
    pub fn map(issue: &str) -> &'static str {
        if issue.contains("SQL") {
            "CWE-89: SQL Injection"
        } else if issue.contains("Command") {
            "CWE-78: OS Command Injection"
        } else if issue.contains("eval") {
            "CWE-94: Code Injection"
        } else {
            "CWE-20: Improper Input Validation"
        }
    }
}
