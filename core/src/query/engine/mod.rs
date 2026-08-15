use crate::findings::finding::Finding;

pub struct QueryEngine;

impl QueryEngine {
    pub fn scan_file(path: &str) -> Vec<Finding> {
        crate::deep::querydsl::scan(path)
    }
}
