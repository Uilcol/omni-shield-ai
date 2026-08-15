#[derive(Debug, Clone)]
pub struct Query {
    pub select: String,
    pub from: String,
    pub condition: String,
}

pub struct QueryEngine;

impl QueryEngine {
    pub fn parse(query: &str) -> Query {
        let lower = query.to_lowercase();

        let mut select = String::new();
        let mut from = String::new();
        let mut condition = String::new();

        if let Some(idx) = lower.find("select") {
            let remain = &query[idx + 6..];

            if let Some(from_idx) = remain.to_lowercase().find("from") {
                select = remain[..from_idx].trim().to_string();

                let remain2 = &remain[from_idx + 4..];

                if let Some(where_idx) = remain2.to_lowercase().find("where") {
                    from = remain2[..where_idx].trim().to_string();

                    condition = remain2[where_idx + 5..].trim().to_string();
                } else {
                    from = remain2.trim().to_string();
                }
            }
        }

        Query {
            select,
            from,
            condition,
        }
    }

    pub fn execute(query: &Query, code: &str) -> Vec<String> {
        let mut out = Vec::new();

        for line in code.lines() {
            let l = line.trim();

            if !query.condition.is_empty() {
                if l.contains(&query.condition) {
                    out.push(l.to_string());
                }
            } else {
                out.push(l.to_string());
            }
        }

        out
    }
}
