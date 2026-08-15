use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SSAState {
    pub versions: HashMap<String, usize>,
}

impl SSAState {
    pub fn new() -> Self {
        Self { versions: HashMap::new() }
    }

    pub fn next(&mut self, var: &str) -> String {
        let v = self.versions.entry(var.to_string()).or_insert(0);
        *v += 1;
        format!("{}_{}", var, v)
    }

    pub fn current(&self, var: &str) -> String {
        match self.versions.get(var) {
            Some(v) => format!("{}_{}", var, v),
            None => format!("{}_0", var),
        }
    }
}
