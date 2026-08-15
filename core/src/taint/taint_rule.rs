#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct TaintRule {
    pub id: String,
    pub source: String,
    pub sink: String,
    pub severity: String,
}

impl TaintRule {
    pub fn new(id: &str, source: &str, sink: &str, severity: &str) -> Self {
        Self {
            id: id.to_string(),
            source: source.to_string(),
            sink: sink.to_string(),
            severity: severity.to_string(),
        }
    }
}
