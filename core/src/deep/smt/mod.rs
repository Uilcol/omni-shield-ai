use z3::{Config, Context, Solver, SatResult, ast::Bool};

pub struct SMTValidator;

impl SMTValidator {
    pub fn validate_path(condition: &str) -> bool {
        println!();
        println!("========== SMT VALIDATOR ==========");
        println!("Constraint: {}", condition);

        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        /*
        Fase inicial:
        placeholder controlado.

        Próxima evolução:
        Parser real:
        user.is_admin && input != sanitized
        ->
        AST lógico
        ->
        SMT AST real
        */

        let symbolic_condition = if condition.contains("false") {
            Bool::from_bool(&ctx, false)
        } else {
            Bool::from_bool(&ctx, true)
        };

        solver.assert(&symbolic_condition);

        let result = solver.check();

        match result {
            SatResult::Sat => {
                println!("Z3 Result: SAT (reachable path)");
                println!("==================================");
                println!();
                true
            }
            _ => {
                println!("Z3 Result: UNSAT (false positive)");
                println!("==================================");
                println!();
                false
            }
        }
    }
}
