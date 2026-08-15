use crate::findings::finding::Finding;
use crate::smt::models::SMTConstraint;

pub struct ConstraintBuilder;

impl ConstraintBuilder {
    pub fn from_finding(finding: &Finding) -> Vec<SMTConstraint> {
        let mut constraints = Vec::new();

        if finding.evidence.contains("user_input") {
            constraints.push(SMTConstraint {
                variable: "input".to_string(),
                operator: "controlled_by_user".to_string(),
                value: "true".to_string(),
            });
        }

        if finding.evidence.contains("eval(") {
            constraints.push(SMTConstraint {
                variable: "sink".to_string(),
                operator: "dangerous_eval".to_string(),
                value: "true".to_string(),
            });
        }

        if finding.evidence.contains("os.system(") {
            constraints.push(SMTConstraint {
                variable: "sink".to_string(),
                operator: "command_execution".to_string(),
                value: "true".to_string(),
            });
        }

        if finding.evidence.contains("SELECT") {
            constraints.push(SMTConstraint {
                variable: "sink".to_string(),
                operator: "sql_execution".to_string(),
                value: "true".to_string(),
            });
        }

        constraints
    }
}
