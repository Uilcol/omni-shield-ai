use crate::analysis::cfg::CFG;
use super::path_state::PathState;
use super::z3_engine::Z3Engine;

pub struct SymbolicExecutor;

impl SymbolicExecutor {
    pub fn execute(cfg: &CFG) {
        let mut states = vec![PathState::new()];

        for node in &cfg.nodes {
            let code = node.code.trim();

            let mut new_states = Vec::new();

            for state in &states {
                // =========================
                // IF BRANCHING
                // =========================
                if code.starts_with("if") {
                    if let Some(cond) = code.strip_prefix("if") {
                        let cond = cond.trim();

                        // TRUE branch
                        let true_state = state.with_constraint(cond);

                        // FALSE branch (negação simples)
                        let false_state = state.with_constraint(&format!("!({})", cond));

                        new_states.push(true_state);
                        new_states.push(false_state);

                        continue;
                    }
                }

                // =========================
                // SINK DETECTION
                // =========================
                if code.contains("eval") || code.contains("exec") {
                    let constraints: Vec<String> = state
                        .constraints
                        .iter()
                        .map(|c| c.expr.clone())
                        .collect();

                    let reachable = Z3Engine::is_reachable(&constraints);

                    if reachable {
                        println!("🔥 REAL VULN (symbolic): {} | constraints: {:?}", code, constraints);
                    } else {
                        println!("✅ PATH descartado: {:?}", constraints);
                    }
                }

                new_states.push(state.clone());
            }

            states = new_states;
        }
    }
}
