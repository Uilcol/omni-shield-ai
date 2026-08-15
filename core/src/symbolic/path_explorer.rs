#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::symbolic_state::SymbolicState;

pub struct PathExplorer;

impl PathExplorer {

    pub fn explore(initial: &SymbolicState) -> Vec<SymbolicState> {

        let mut states = Vec::new();

        // estado base
        states.push(initial.clone());

        // 🔥 simulação de branch
        if initial.path_constraints.len() < 5 {

            let mut branch_true = initial.clone();
            branch_true.path_constraints.push("branch_true".to_string());

            let mut branch_false = initial.clone();
            branch_false.path_constraints.push("branch_false".to_string());

            states.push(branch_true);
            states.push(branch_false);
        }

        states
    }
}
