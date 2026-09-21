use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sanitizers {
    functions: HashSet<String>,
}

impl Sanitizers {
    pub fn new() -> Self {
        let functions = [
            // Generic
            "sanitize",
            "escape",
            "clean",
            "validate",
            "normalize",
            // HTML / XSS
            "html.escape",
            "html_escape",
            "markupsafe.escape",
            "bleach.clean",
            // SQL
            "sql_escape",
            "quote",
            "sqlalchemy.text",
            // Shell / command
            "shlex.quote",
            // URL / encoding
            "urllib.parse.quote",
            "urlencode",
        ]
        .into_iter()
        .map(str::to_ascii_lowercase)
        .collect();

        Self { functions }
    }

    pub fn is_sanitizer(&self, name: &str) -> bool {
        let normalized = name.trim().trim_end_matches(';').to_ascii_lowercase();

        if self.functions.contains(&normalized) {
            return true;
        }

        self.functions.iter().any(|sanitizer| {
            normalized.starts_with(&format!("{sanitizer}("))
                || normalized.contains(&format!(".{sanitizer}("))
        })
    }
}
