use z3::{Config, Context, Solver, ast::Bool};

pub struct Z3Engine;

impl Z3Engine {
    pub fn check_vulnerability(conditions: Vec<String>) -> bool {
        let mut config = Config::new();
        config.set_timeout_msec(5000);

        let ctx = Context::new(&config);
        let solver = Solver::new(&ctx);

        let mut has_eval = false;
        let mut has_input = false;

        for cond in conditions {
            if cond.contains("input") {
                has_input = true;
            }
            if cond.contains("eval") || cond.contains("exec") {
                has_eval = true;
            }
        }

        // modelo simbólico simples (expandiremos depois)
        let input = Bool::new_const(&ctx, "user_input");
        let dangerous = Bool::new_const(&ctx, "dangerous_sink");

        if has_input {
            solver.assert(&input);
        }

        if has_eval {
            solver.assert(&dangerous);
        }

        // regra: input AND dangerous => vulnerável
        let vuln = Bool::and(&ctx, &[&input, &dangerous]);
        solver.assert(&vuln);

        match solver.check() {
            z3::SatResult::Sat => true,
            _ => false,
        }
    }
}
