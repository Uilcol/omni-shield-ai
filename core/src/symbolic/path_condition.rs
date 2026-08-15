use crate::symbolic::constraint::Constraint;

#[derive(Debug, Clone)]
pub struct PathCondition {
    pub constraints: Vec<Constraint>,
}

impl PathCondition {
    pub fn new() -> Self {
        Self {
            constraints: vec![],
        }
    }

    pub fn add(&mut self, c: Constraint) {
        self.constraints.push(c);
    }
}
