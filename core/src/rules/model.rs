#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub source: String,
    pub sink: String,
    pub description: String,
    pub severity: String,
    pub require_sanitized: bool,
}
