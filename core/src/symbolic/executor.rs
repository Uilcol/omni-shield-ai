use crate::symbolic::constraint::Constraint;
use crate::symbolic::path_condition::PathCondition;
use crate::symbolic::value::SymbolicValue;

pub struct SymbolicExecutor;

impl SymbolicExecutor {
    pub fn execute() -> Vec<PathCondition> {
        let mut paths = vec![];

        // Exemplo: if (x > 0)
        let mut pc1 = PathCondition::new();
        pc1.add(Constraint::Gt(
            SymbolicValue::Var("x".to_string()),
            SymbolicValue::Const(0),
        ));

        let mut pc2 = PathCondition::new();
        pc2.add(Constraint::Lt(
            SymbolicValue::Var("x".to_string()),
            SymbolicValue::Const(0),
        ));

        paths.push(pc1);
        paths.push(pc2);

        paths
    }
}
