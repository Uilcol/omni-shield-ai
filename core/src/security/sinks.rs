use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Sinks {
    functions: HashSet<String>,
}

impl Sinks {
    pub fn new() -> Self {
        let functions = [
            // Code execution
            "eval",
            "exec",
            // Command execution
            "system",
            "os.system",
            "subprocess.call",
            "subprocess.run",
            "subprocess.popen",
            "spawn",
            "child_process.exec",
            "child_process.spawn",
            // SQL
            "execute",
            "query",
            "cursor.execute",
            "connection.execute",
            "session.execute",
            "engine.execute",
            // File / filesystem
            "open",
            "path.open",
            // Web/template output
            "render",
            "render_template",
            "template.render",
        ]
        .into_iter()
        .map(str::to_ascii_lowercase)
        .collect();

        Self { functions }
    }

    pub fn is_sink(&self, name: &str) -> bool {
        let normalized = name.trim().trim_end_matches(';').to_ascii_lowercase();

        if self.functions.contains(&normalized) {
            return true;
        }

        self.functions.iter().any(|sink| {
            normalized == *sink
                || normalized.starts_with(&format!("{sink}("))
                || normalized.ends_with(&format!(".{sink}"))
                || normalized.contains(&format!(".{sink}("))
                || normalized.ends_with(&format!("::{sink}"))
        })
    }
}
