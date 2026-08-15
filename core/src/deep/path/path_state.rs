use super::constraint::Constraint;

#[derive(Debug, Clone)]
pub struct PathState {
    pub constraints: Vec<Constraint>,
}

impl PathState {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }

    pub fn with_constraint(&self, expr: &str) -> Self {
        let mut new = self.clone();
        new.constraints.push(Constraint::new(expr));
        new
    }
}
