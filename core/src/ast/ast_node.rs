#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: String,
    pub node_type: String, // ou enum
    pub text: String,
    pub children: Vec<AstNode>,
}
