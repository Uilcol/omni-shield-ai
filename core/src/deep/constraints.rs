pub struct ConstraintBuilder;

impl ConstraintBuilder {
    pub fn build(symbolic: Vec<String>) -> Vec<String> {
        symbolic
            .into_iter()
            .map(|c| format!("constraint({})", c))
            .collect()
    }
}
