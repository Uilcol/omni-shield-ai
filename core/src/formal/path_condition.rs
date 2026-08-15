#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct PathCondition {
    pub constraints: Vec<bool>,
}

impl PathCondition {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
        }
    }

    pub fn add(&mut self, cond: bool) {
        self.constraints.push(cond);
    }

    pub fn is_satisfied(&self) -> bool {
        self.constraints.iter().all(|c| *c)
    }
}
