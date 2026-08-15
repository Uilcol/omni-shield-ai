#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashMap;
pub struct RuleCache {
    pub rules: HashMap<String, String>,
}
impl RuleCache {
    pub fn new() -> Self {
        RuleCache {
            rules: HashMap::new(),
        }
    }
    pub fn get(&mut self, _key: &str) -> String {
        todo!()
    }
}
