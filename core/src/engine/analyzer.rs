#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;

use crate::rules::matcher::RuleMatcher;
use crate::rules::rule::Rule;
use crate::finding::Finding;

pub struct Analyzer {
    pub rules: Vec<Rule>,
}

impl Analyzer {
    pub fn new(rules: Vec<Rule>) -> Self { Self { rules } }

    pub fn analyze(&self, code: &str, file: &str) -> Vec<Finding> {
        RuleMatcher::match_rules(code, &self.rules, file)
    }
}

    let code = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    RuleMatcher::match_rules(&code, &[], path.to_str().unwrap_or(""))
}
