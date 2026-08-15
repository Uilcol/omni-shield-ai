#[derive(Debug, Clone)]
pub struct QueryPack {
    pub name: String,
    pub rules: Vec<String>,
}

impl QueryPack {
    pub fn default_security_pack() -> Self {
        Self {
            name: "security-pack".to_string(),
            rules: vec![
                "sql_injection".to_string(),
                "command_injection".to_string(),
                "hardcoded_secret".to_string(),
                "unsafe_eval".to_string(),
                "path_traversal".to_string(),
            ],
        }
    }
}
