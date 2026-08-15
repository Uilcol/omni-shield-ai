pub struct RuleEngine;

impl RuleEngine {
    pub fn execute(rule: &str, code: &str) -> bool {
        match rule {
            "sql_injection" => code.contains("SELECT") && code.contains("+"),
            "command_injection" => code.contains("system(") || code.contains("exec("),
            "hardcoded_secret" => code.contains("password") || code.contains("api_key"),
            "unsafe_eval" => code.contains("eval("),
            "path_traversal" => code.contains("../"),
            _ => false,
        }
    }
}
