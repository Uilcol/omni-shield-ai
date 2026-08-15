#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

pub struct TaintTracker {
    sources: HashSet<String>,
    sinks: HashSet<String>,
    sanitizers: HashSet<String>,
}

impl TaintTracker {
    pub fn new() -> Self {
        Self {
            sources: HashSet::new(),
            sinks: HashSet::new(),
            sanitizers: HashSet::new(),
        }
    }

    pub fn add_source(&mut self, name: &str) {
        self.sources.insert(name.to_string());
    }

    pub fn add_sink(&mut self, name: &str) {
        self.sinks.insert(name.to_string());
    }

    pub fn add_sanitizer(&mut self, name: &str) {
        self.sanitizers.insert(name.to_string());
    }

    pub fn is_source(&self, name: &str) -> bool {
        self.sources.contains(name)
    }

    pub fn is_sink(&self, name: &str) -> bool {
        self.sinks.contains(name)
    }

    pub fn is_sanitizer(&self, name: &str) -> bool {
        self.sanitizers.contains(name)
    }
}
