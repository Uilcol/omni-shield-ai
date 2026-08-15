#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::cwe::CWE;

pub struct CweDatabase {
    pub entries: Vec<CWE>,
}

impl CweDatabase {
    pub fn new() -> Self {
        Self {
            entries: vec![
                CWE::new(
                    "CWE-78",
                    "Command Injection",
                    "Improper neutralization of special elements in OS commands.",
                ),
                CWE::new(
                    "CWE-94",
                    "Code Injection",
                    "Improper control of code generation or execution.",
                ),
                CWE::new(
                    "CWE-89",
                    "SQL Injection",
                    "Improper neutralization of special elements in SQL commands.",
                ),
                CWE::new(
                    "CWE-79",
                    "Cross-Site Scripting",
                    "Improper neutralization of input during web page generation.",
                ),
                CWE::new(
                    "CWE-22",
                    "Path Traversal",
                    "Improper limitation of pathname to restricted directory.",
                ),
            ],
        }
    }

    pub fn find(&self, id: &str) -> Option<&CWE> {
        self.entries.iter().find(|c| c.id == id)
    }
}
