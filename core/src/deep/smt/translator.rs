use crate::deep::ast::nodes::ASTNode;

pub struct SMTTranslator;

impl SMTTranslator {
    pub fn to_smt(ast: &Vec<ASTNode>) -> Vec<String> {
        let mut constraints = Vec::new();

        for node in ast {
            match node {
                ASTNode::If { condition } => {
                    constraints.push(condition.clone());
                }
                ASTNode::Assign { var, value } => {
                    constraints.push(format!("{} == {}", var, value));
                }
                _ => {}
            }
        }

        constraints
    }
}
