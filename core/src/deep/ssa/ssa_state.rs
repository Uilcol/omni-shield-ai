use std::collections::HashMap;

#[derive(Debug)]
pub struct SSAState {
    pub versions: HashMap<String, usize>,
}

impl SSAState {
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
        }
    }

    pub fn next_version(&mut self, var: &str) -> usize {
        let counter = self.versions.entry(var.to_string()).or_insert(0);
        *counter += 1;
        *counter
    }

    pub fn current(&self, var: &str) -> usize {
        *self.versions.get(var).unwrap_or(&0)
    }
}
