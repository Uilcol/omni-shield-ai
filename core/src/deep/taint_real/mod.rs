use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct TaintResult {
    pub source: String,
    pub sink: String,
    pub path: Vec<String>,
    pub exploitable: bool,
}

pub struct TaintEngineReal;

impl TaintEngineReal {
    pub fn known_sources() -> HashSet<String> {
        let mut s = HashSet::new();

        s.insert("request.params".to_string());
        s.insert("request.body".to_string());
        s.insert("stdin".to_string());
        s.insert("env".to_string());
        s.insert("db.read".to_string());
        s.insert("file.read".to_string());

        s
    }

    pub fn known_sinks() -> HashSet<String> {
        let mut s = HashSet::new();

        s.insert("sql.query".to_string());
        s.insert("exec".to_string());
        s.insert("eval".to_string());
        s.insert("shell".to_string());
        s.insert("file.write".to_string());
        s.insert("deserialize".to_string());
        s.insert("template.render".to_string());

        s
    }

    pub fn known_sanitizers() -> HashSet<String> {
        let mut s = HashSet::new();

        s.insert("prepared_statement".to_string());
        s.insert("escape_sql".to_string());
        s.insert("validate_input".to_string());
        s.insert("allowlist".to_string());

        s
    }

    pub fn analyze(code: &str) -> Vec<TaintResult> {
        let sources = Self::known_sources();
        let sinks = Self::known_sinks();
        let sanitizers = Self::known_sanitizers();

        let mut findings = Vec::new();
        let mut discovered: HashMap<String, bool> = HashMap::new();

        for line in code.lines() {
            let l = line.trim();

            for src in &sources {
                if l.contains(src) {
                    discovered.insert(src.clone(), true);
                }
            }

            let mut sanitized = false;
            for san in &sanitizers {
                if l.contains(san) {
                    sanitized = true;
                }
            }

            for sink in &sinks {
                if l.contains(sink) {
                    for src in discovered.keys() {
                        findings.push(TaintResult {
                            source: src.clone(),
                            sink: sink.clone(),
                            path: vec![
                                src.clone(),
                                "propagation".to_string(),
                                sink.clone(),
                            ],
                            exploitable: !sanitized,
                        });
                    }
                }
            }
        }

        findings
    }
}
