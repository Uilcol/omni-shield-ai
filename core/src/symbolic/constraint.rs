use crate::symbolic::value::SymbolicValue;

#[derive(Debug, Clone)]
pub enum Constraint {
    Eq(SymbolicValue, SymbolicValue),
    Neq(SymbolicValue, SymbolicValue),
    Gt(SymbolicValue, SymbolicValue),
    Lt(SymbolicValue, SymbolicValue),
}
