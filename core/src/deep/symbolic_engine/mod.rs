use std::collections::HashMap;
use crate::analysis::ast::ASTNode;

pub struct SymbolicExecutor;

impl SymbolicExecutor {
    pub fn execute(ast: &Vec<ASTNode>) -> HashMap<String, String> {
        let mut state = HashMap::new();

        for node in ast {
            match node {
                ASTNode::Assign { var, value } => {
                    state.insert(var.clone(), value.clone());
                }
                _ => {}
            }
        }

        state
    }
}
