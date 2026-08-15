use crate::finding::Finding;
use crate::rules::rule::Rule;

pub struct RuleMatcher;

impl RuleMatcher {
    pub fn new(_rules: Vec<Rule>) -> Self {
        RuleMatcher
    }

    pub fn match_rules_for(&self, _code: &str, _file: &str) -> Vec<Finding> {
        vec![]
    }
}
