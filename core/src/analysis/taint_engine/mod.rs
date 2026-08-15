pub struct TaintEngine;

impl TaintEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze<T>(&self, _cfg: &T) -> Vec<()> {
        vec![]
    }
}
