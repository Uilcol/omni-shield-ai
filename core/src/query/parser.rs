pub struct Query {
    pub pattern: String,
}

pub struct QueryParser;

impl QueryParser {
    pub fn parse(q: &str) -> Query {
        Query {
            pattern: q.to_string(),
        }
    }
}
