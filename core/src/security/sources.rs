#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sources {
    functions: HashSet<String>,
}

impl Sources {
    pub fn new() -> Self {
        let mut functions = HashSet::new();

        // Web input
        functions.insert("input".into());
        functions.insert("request.get".into());
        functions.insert("request.post".into());

        // Python
        functions.insert("sys.argv".into());

        // NodeJS
        functions.insert("req.body".into());
        functions.insert("req.query".into());

        // Rust
        functions.insert("std::env::var".into());

        Self { functions }
    }

    pub fn is_source(&self, name: &str) -> bool {
        self.functions.contains(name)
    }
}
