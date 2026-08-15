use crate::findings::finding::Finding;
use crate::query::engine::QueryEngine;

pub fn scan_file(path: &str) -> Vec<Finding> {
    QueryEngine::scan_file(path)
}
