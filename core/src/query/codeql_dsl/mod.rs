#[derive(Debug, Clone)]
pub struct Query {
    pub select: String,
    pub from: String,
    pub where_clause: String,
}

pub struct CodeQLDSL;

impl CodeQLDSL {
    pub fn parse(input: &str) -> Option<Query> {
        let normalized = input.replace("\n", " ");

        let parts: Vec<&str> = normalized.split("where").collect();

        if parts.len() != 2 {
            return None;
        }

        let left = parts[0];
        let right = parts[1];

        let left_parts: Vec<&str> = left.split("from").collect();

        if left_parts.len() != 2 {
            return None;
        }

        let select = left_parts[0].replace("select", "").trim().to_string();

        let from = left_parts[1].trim().to_string();

        Some(Query {
            select,
            from,
            where_clause: right.trim().to_string(),
        })
    }
}
