#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::cwe_database::CweDatabase;

#[allow(dead_code)]
pub struct CweMapper {
    db: CweDatabase,
}

impl CweMapper {
    pub fn new(db: CweDatabase) -> Self {
        Self { db }
    }

    pub fn map(&self, rule: &str) -> Option<String> {
        // futuramente consulta DB
        match rule {
            "PY-EVAL-001" => Some("CWE-95".to_string()),
            "PY-SECRET-001" => Some("CWE-798".to_string()),
            _ => None,
        }
    }
}
