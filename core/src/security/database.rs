use crate::security::sanitizers::Sanitizers;
use crate::security::sinks::Sinks;
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::security::sources::Sources;

#[derive(Debug, Clone)]
pub struct SecurityDatabase {
    pub sources: Sources,
    pub sinks: Sinks,
    pub sanitizers: Sanitizers,
}

impl SecurityDatabase {
    pub fn new() -> Self {
        Self {
            sources: Sources::new(),
            sinks: Sinks::new(),
            sanitizers: Sanitizers::new(),
        }
    }

    pub fn is_source(&self, function: &str) -> bool {
        self.sources.is_source(function)
    }

    pub fn is_sink(&self, function: &str) -> bool {
        self.sinks.is_sink(function)
    }

    pub fn is_sanitizer(&self, function: &str) -> bool {
        self.sanitizers.is_sanitizer(function)
    }
}
