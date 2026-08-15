#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sinks {
    functions: HashSet<String>,
}

impl Sinks {
    pub fn new() -> Self {
        let mut functions = HashSet::new();

        // SQL
        functions.insert("execute".into());
        functions.insert("query".into());

        // Command execution
        functions.insert("system".into());
        functions.insert("exec".into());
        functions.insert("spawn".into());

        // File
        functions.insert("open".into());

        // Web
        functions.insert("render".into());

        Self { functions }
    }

    pub fn is_sink(&self, name: &str) -> bool {
        self.functions.contains(name)
    }
}
