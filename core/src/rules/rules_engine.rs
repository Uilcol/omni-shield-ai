use crate::finding::Finding;
use crate::rules::matcher::RuleMatcher;
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::rules::rule::Rule;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    pub fn with_rules(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn execute(&self, code: &str, file: &str) -> Vec<Finding> {
        RuleMatcher::new(self.rules.clone()).match_rules_for(code, file)
    }
}
