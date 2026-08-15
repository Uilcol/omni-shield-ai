use crate::symbolic::path_condition::PathCondition;

pub struct VulnDetector;

impl VulnDetector {
    pub fn detect(pc: &PathCondition) -> Vec<String> {
        let mut vulns = vec![];

        for c in &pc.constraints {
            let s = format!("{:?}", c);

            if s.contains("Var(\"user_input\")") {
                vulns.push("Potential SQL Injection".to_string());
            }
        }

        vulns
    }
}
