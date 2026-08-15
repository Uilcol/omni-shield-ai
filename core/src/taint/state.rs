#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct TaintState {
    tainted: HashSet<String>,
}

impl TaintState {
    pub fn new() -> Self {
        Self {
            tainted: HashSet::new(),
        }
    }

    pub fn taint(&mut self, var: impl Into<String>) {
        self.tainted.insert(var.into());
    }

    pub fn is_tainted(&self, var: impl AsRef<str>) -> bool {
        self.tainted.contains(var.as_ref())
    }
}
