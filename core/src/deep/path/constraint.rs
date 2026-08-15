#[derive(Debug, Clone)]
pub struct Constraint {
    pub expr: String,
}

impl Constraint {
    pub fn new(expr: &str) -> Self {
        Self {
            expr: expr.to_string(),
        }
    }
}
