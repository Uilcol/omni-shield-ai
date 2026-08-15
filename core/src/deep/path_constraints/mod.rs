use z3::{
    ast::{Ast, Bool, Int},
    Config,
    Context,
    SatResult,
    Solver,
};

#[derive(Debug, Clone)]
pub struct Constraint {
    pub variable: String,
    pub operator: String,
    pub value: i64,
}

pub struct PathConstraintEngine;

impl PathConstraintEngine {
    pub fn validate(
        constraints: &[Constraint],
    ) -> bool {
        let cfg = Config::new();
        let ctx = Context::new(&cfg);
        let solver = Solver::new(&ctx);

        let input =
            Int::new_const(&ctx, "input");

        for c in constraints {
            match c.operator.as_str() {
                "==" => {
                    solver.assert(
                        &input._eq(
                            &Int::from_i64(
                                &ctx,
                                c.value,
                            ),
                        ),
                    );
                }

                "!=" => {
                    let neq: Bool =
                        input
                            ._eq(
                                &Int::from_i64(
                                    &ctx,
                                    c.value,
                                ),
                            )
                            .not();

                    solver.assert(&neq);
                }

                ">" => {
                    solver.assert(
                        &input.gt(
                            &Int::from_i64(
                                &ctx,
                                c.value,
                            ),
                        ),
                    );
                }

                "<" => {
                    solver.assert(
                        &input.lt(
                            &Int::from_i64(
                                &ctx,
                                c.value,
                            ),
                        ),
                    );
                }

                ">=" => {
                    solver.assert(
                        &input.ge(
                            &Int::from_i64(
                                &ctx,
                                c.value,
                            ),
                        ),
                    );
                }

                "<=" => {
                    solver.assert(
                        &input.le(
                            &Int::from_i64(
                                &ctx,
                                c.value,
                            ),
                        ),
                    );
                }

                _ => {}
            }
        }

        matches!(
            solver.check(),
            SatResult::Sat
        )
    }
}
