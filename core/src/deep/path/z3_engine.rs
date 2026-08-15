use z3::{Config, Context, Solver, ast::Bool};

pub struct Z3Engine;

impl Z3Engine {
    pub fn check(condition: &str) -> bool {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        // simplificação: qualquer condição vira bool simbólico
        let cond = Bool::new_const(&ctx, condition);

        solver.assert(&cond);

        match solver.check() {
            z3::SatResult::Sat => true,
            _ => false
        }
    }
}
