#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::rules::matcher::RuleMatcher;
use crate::rules::rule::Rule;
use crate::finding::Finding;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self { Self { rules: Vec::new() } }
    pub fn with_rules(rules: Vec<Rule>) -> Self { Self { rules } }

    /// Recebe (code, file) — 2 argumentos
    pub fn run(&self, code: &str, file: &str) -> Vec<Finding> {
        RuleMatcher::match_rules(code, &self.rules, file)
    }
}
