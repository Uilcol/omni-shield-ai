use crate::deep::z3_solver::Z3Engine;

pub struct ConstraintSolver;

impl ConstraintSolver {
    pub fn solve(constraints: Vec<String>) -> bool {
        Z3Engine::check_vulnerability(constraints)
    }
}
