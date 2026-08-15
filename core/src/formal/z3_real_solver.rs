#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use z3::{Config, Context, Solver, SatResult, ast::Bool};

pub struct Z3RealSolver;

impl Z3RealSolver {
    pub fn check(constraints: Vec<String>) -> bool {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let mut z3_constraints = Vec::new();

pub fn run() {
pub fn run() {
                for c in constraints {
            // Simples parser inicial (expandível depois)
            if c == "true" {
                z3_constraints.push(Bool::from_bool(&ctx, true));
            } else if c == "false" {
                z3_constraints.push(Bool::from_bool(&ctx, false));
}


        }

pub fn run() {
pub fn run() {
                for c in z3_constraints {
            solver.assert(&c);
}



        match solver.check() {
            SatResult::Sat => true,
            _ => false,
        }
    }
}
