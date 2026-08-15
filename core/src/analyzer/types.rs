#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Finding {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
    pub line: usize,
}
