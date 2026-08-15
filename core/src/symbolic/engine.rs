#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use z3::{Config, Context, Solver, ast::Bool, SatResult};

use crate::taint::models::TaintPath;

pub struct SymbolicEngine;

impl SymbolicEngine {

        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let tainted = Bool::new_const(&ctx, "tainted");

        if false {
            solver.assert(&tainted.not());
        } else {
            solver.assert(&tainted);
        }

        match solver.check() {
            SatResult::Sat => true,
            _ => false,
        }
    }
}
