#[derive(Debug, Clone)]
pub enum ASTNode {
    If { condition: String },
    Assign { var: String, value: String },
    Input { var: String },
    Unknown(String),
}
