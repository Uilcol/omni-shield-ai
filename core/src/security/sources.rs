use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sources {
    functions: HashSet<String>,
}

impl Sources {
    pub fn new() -> Self {
        let functions = [
            // Generic / stdin
            "input",
            "read",
            "stdin",
            "argv",
            // Python / Flask / Django / Starlette
            "request",
            "request.get",
            "request.post",
            "request.args",
            "request.form",
            "request.json",
            "request.values",
            "request.files",
            "request.body",
            "request.get_json",
            "request.query_params",
            "request.path_params",
            "request.headers",
            "request.cookies",
            "request.GET",
            "request.POST",
            // Node / Express
            "req.body",
            "req.query",
            "req.params",
            "req.headers",
            "req.cookies",
            // Rust / process environment
            "std::env::var",
            "std::env::args",
        ]
        .into_iter()
        .map(str::to_ascii_lowercase)
        .collect();

        Self { functions }
    }

    pub fn is_source(&self, name: &str) -> bool {
        let normalized = name.trim().trim_end_matches(';').to_ascii_lowercase();

        if self.functions.contains(&normalized) {
            return true;
        }

        self.functions.iter().any(|source| {
            normalized.starts_with(&format!("{source}("))
                || normalized.starts_with(&format!("{source}."))
                || normalized.ends_with(&format!(".{source}"))
                || normalized.ends_with(&format!(".{source}()"))
        })
    }
}
