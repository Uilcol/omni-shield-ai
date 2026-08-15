#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;
use syn;

pub struct RustParser;

impl RustParser {
    pub fn parse(path: &str) -> Option<syn::File> {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return None,
        };

        match syn::parse_file(&content) {
            Ok(ast) => Some(ast),
            Err(_) => None,
        }
    }
}
