use std::collections::HashMap;
use crate::analysis::cfg::CFG;
use super::state::SSAState;

pub struct SSABuilder;

impl SSABuilder {
    pub fn build(cfg: &CFG) -> HashMap<usize, Vec<String>> {
        let mut result = HashMap::new();
        let mut state = SSAState::new();

        for node in &cfg.nodes {
            let mut stmts = vec![];

            // exemplo simplificado: "x = y"
            if node.code.contains("=") {
                let parts: Vec<&str> = node.code.split('=').collect();
                if parts.len() == 2 {
                    let var = parts[0].trim();
                    let val = parts[1].trim();

                    let new_var = state.next(var);
                    stmts.push(format!("{} = {}", new_var, val));
                }
            }

            result.insert(node.id, stmts);
        }

        result
    }

    pub fn insert_phi(
        incoming: Vec<SSAState>,
        var: &str
    ) -> String {
        let versions: Vec<String> = incoming
            .iter()
            .map(|s| s.current(var))
            .collect();

        format!("{} = φ({})", var, versions.join(", "))
    }
}
