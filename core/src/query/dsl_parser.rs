#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::query::dsl_query::Query;

pub struct DSLParser;

impl DSLParser {
    pub fn parse(input: &str) -> Query {
        let mut query = Query::new();

pub fn run() {
pub fn run() {
                for line in input.lines() {
            let line = line.trim();

            if line.contains("source") {
                if let Some(val) = line.split('=').nth(1) {
                    query.source = val.trim().replace("\"", "");
}


            }

            if line.contains("sink") {
                if let Some(val) = line.split('=').nth(1) {
                    query.sink = val.trim().replace("\"", "");
                }
            }

            if line.contains("NOT sanitized") {
                query.require_unsanitized = true;
            }
        }

        query
    }
}
