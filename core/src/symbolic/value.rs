#[derive(Debug, Clone)]
pub enum SymbolicValue {
    Var(String),
    Const(i32),
    Unknown,
}
