use crate::smt::models::{SMTConstraint, SMTValidationResult};

pub struct SMTSolver;

impl SMTSolver {
    pub fn solve(constraints: &[SMTConstraint]) -> SMTValidationResult {
        let has_user_control = constraints
            .iter()
            .any(|c| c.operator == "controlled_by_user");

        let has_dangerous_sink = constraints.iter().any(|c| {
            c.operator == "dangerous_eval"
                || c.operator == "command_execution"
                || c.operator == "sql_execution"
        });

        if has_user_control && has_dangerous_sink {
            SMTValidationResult {
                satisfiable: true,
                validated: true,
                confidence_boost: 0.15,
                reason: "User-controlled input reaches exploitable sink".to_string(),
            }
        } else if has_dangerous_sink {
            SMTValidationResult {
                satisfiable: true,
                validated: false,
                confidence_boost: 0.05,
                reason: "Dangerous sink found, but source validation incomplete".to_string(),
            }
        } else {
            SMTValidationResult {
                satisfiable: false,
                validated: false,
                confidence_boost: -0.25,
                reason: "No exploitable path proven".to_string(),
            }
        }
    }
}
