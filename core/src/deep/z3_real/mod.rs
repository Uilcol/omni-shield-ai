use z3::{
    ast::{Ast, Bool, Int},
    Config,
    Context,
    SatResult,
    Solver,
};

#[derive(Debug, Clone)]
pub struct PathConstraint {
    pub variable: String,
    pub op: String,
    pub value: i64,
}

#[derive(Debug, Clone)]
pub struct Z3ValidationResult {
    pub satisfiable: bool,
    pub exploitable: bool,
}

pub struct Z3FalsePositiveKiller;

impl Z3FalsePositiveKiller {
    pub fn validate(constraints: Vec<PathConstraint>) -> Z3ValidationResult {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let input = Int::new_const(&ctx, "input");

        for c in constraints {
            match c.op.as_str() {
                ">" => solver.assert(&input.gt(&Int::from_i64(&ctx, c.value))),
                "<" => solver.assert(&input.lt(&Int::from_i64(&ctx, c.value))),
                ">=" => solver.assert(&input.ge(&Int::from_i64(&ctx, c.value))),
                "<=" => solver.assert(&input.le(&Int::from_i64(&ctx, c.value))),

                "==" => {
                    let eq_ast: Bool = input._eq(&Int::from_i64(&ctx, c.value));
                    solver.assert(&eq_ast);
                }

                "!=" => {
                    let neq_ast: Bool =
                        input._eq(&Int::from_i64(&ctx, c.value)).not();

                    solver.assert(&neq_ast);
                }

                _ => {}
            }
        }

        match solver.check() {
            SatResult::Sat => Z3ValidationResult {
                satisfiable: true,
                exploitable: true,
            },

            SatResult::Unsat => Z3ValidationResult {
                satisfiable: false,
                exploitable: false,
            },

            SatResult::Unknown => Z3ValidationResult {
                satisfiable: false,
                exploitable: false,
            },
        }
    }
}
