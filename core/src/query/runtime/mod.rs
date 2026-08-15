use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rule: String,
    pub matches: Vec<String>,
}

pub struct QueryRuntime;

impl QueryRuntime {
    pub fn execute(
        query: &str,
        code: &str,
    ) -> QueryResult {
        let mut matches = Vec::new();

        if query.contains("eval") {
            for line in code.lines() {
                if line.contains("eval(") {
                    matches.push(
                        line.trim().to_string(),
                    );
                }
            }
        }

        if query.contains("exec") {
            for line in code.lines() {
                if line.contains("exec(") {
                    matches.push(
                        line.trim().to_string(),
                    );
                }
            }
        }

        QueryResult {
            rule: query.to_string(),
            matches,
        }
    }
}
