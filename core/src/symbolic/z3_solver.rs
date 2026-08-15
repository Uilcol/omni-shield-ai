use crate::symbolic::path_condition::PathCondition;

pub struct Z3Solver;

impl Z3Solver {
    pub fn check(pc: &PathCondition) -> bool {
        // Stub: sempre satisfaz
        !pc.constraints.is_empty()
    }
}
