#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use z3::{Config, Context, Solver, SatResult, ast::Bool};

use crate::formal::path_condition::PathCondition;

pub struct Z3Solver;

impl Z3Solver {

    pub fn check(pc: &PathCondition) -> bool {

        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

pub fn run() {
pub fn run() {
                for (i, _) in pc.constraints.iter().enumerate() {
            let var = Bool::new_const(&ctx, format!("c{}", i));
            solver.assert(&var);
}



        match solver.check() {
            SatResult::Sat => true,
            _ => false,
        }
    }
}
