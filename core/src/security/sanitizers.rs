#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sanitizers {
    functions: HashSet<String>,
}

impl Sanitizers {
    pub fn new() -> Self {
        let mut functions = HashSet::new();

        functions.insert("escape".into());
        functions.insert("sanitize".into());
        functions.insert("html_escape".into());
        functions.insert("sql_escape".into());

        Self { functions }
    }

    pub fn is_sanitizer(&self, name: &str) -> bool {
        self.functions.contains(name)
    }
}
