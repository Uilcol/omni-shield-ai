#[derive(Debug, Clone)]
pub struct SMTConstraint {
    pub variable: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct SMTValidationResult {
    pub satisfiable: bool,
    pub validated: bool,
    pub confidence_boost: f64,
    pub reason: String,
}
