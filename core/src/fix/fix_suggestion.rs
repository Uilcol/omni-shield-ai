#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct FixSuggestion {

    pub rule_id: String,
    pub recommendation: String,

}

impl FixSuggestion {

    pub fn new(rule: &str, recommendation: &str) -> Self {

        Self {

            rule_id: rule.to_string(),
            recommendation: recommendation.to_string(),
            security_path: None,

        }

    }
}
