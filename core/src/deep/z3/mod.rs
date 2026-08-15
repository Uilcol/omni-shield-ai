use z3::{ast::Bool, Config, Context, Solver};

pub struct Z3Engine;

impl Z3Engine {
    pub fn check() {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let a = Bool::new_const(&ctx, "a");
        solver.assert(&a);

        let _ = solver.check();
    }
}
