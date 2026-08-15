#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::symbolic_variable::SymbolicVariable;

#[derive(Debug, Clone)]
pub struct SymbolicState {

    pub variables: Vec<SymbolicVariable>,
    pub path_constraints: Vec<String>,

}

impl SymbolicState {

    pub fn new() -> Self {

        Self {

            variables: Vec::new(),
            path_constraints: Vec::new(),

        }

    }

    pub fn add_variable(&mut self, var: SymbolicVariable) {

        self.variables.push(var);

    }

    pub fn add_constraint(&mut self, constraint: &str) {

        self.path_constraints.push(constraint.to_string());

    }
}
