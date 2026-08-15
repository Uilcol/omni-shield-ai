#[derive(Debug, Clone)]
pub struct SecurityPolicy {
    pub name: String,
    pub minimum_score_block: u32,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn default_policy() -> SecurityPolicy {
        SecurityPolicy {
            name: "enterprise-security-policy".to_string(),
            minimum_score_block: 8,
        }
    }
}
