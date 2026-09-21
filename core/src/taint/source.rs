#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct Source;

impl Source {
    pub fn is_source(node: &str) -> bool {
        let sources = ["input", "read", "request", "params", "argv"];

        sources.iter().any(|s| node.contains(s))
    }
}

pub struct SourceDetector;

impl SourceDetector {
    pub fn is_source(name: &str) -> bool {
        crate::security::database::SecurityDatabase::new().is_source(name)
    }
}
