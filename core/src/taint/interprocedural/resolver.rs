#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashMap;

#[derive(Debug)]
pub struct CallResolver {
    calls: HashMap<String, Vec<String>>,
}

impl CallResolver {
    pub fn new() -> Self {
        Self {
            calls: HashMap::new(),
        }
    }

    pub fn register_call(&mut self, function: &str, callee: &str) {
        self.calls
            .entry(function.to_string())
            .or_insert(Vec::new())
            .push(callee.to_string());
    }

    pub fn resolve(&self, function: &str) -> Option<&Vec<String>> {
        self.calls.get(function)
    }
}
