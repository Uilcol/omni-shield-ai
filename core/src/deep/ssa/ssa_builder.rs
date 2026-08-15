use crate::analysis::ast::ASTNode;
use super::SSAState;

pub struct SSABuilder;

impl SSABuilder {
    pub fn transform(ast: &Vec<ASTNode>) -> Vec<String> {
        let mut state = SSAState::new();
        let mut output = vec![];

        for node in ast {
            match node {
                ASTNode::Assign { var, value } => {
                    let version = state.next_version(var);
                    let new_var = format!("{}_{}", var, version);
                    output.push(format!("{} = {}", new_var, value));
                }
                _ => {}
            }
        }

        output
    }
}
