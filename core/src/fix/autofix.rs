#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct AutoFix;

impl AutoFix {
    pub fn suggest(node: &str) -> Option<String> {
        if node.contains("exec") {
            return Some("Use safe_exec() instead".to_string());
        }
        if node.contains("sql") {
            return Some("Use prepared statements".to_string());
        }
        None
    }
}
