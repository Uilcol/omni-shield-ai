#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::rules::rule::Rule;

pub struct LanguageRuleFilter;

impl LanguageRuleFilter {

    pub fn filter(rules: &[Rule], language: &str) -> Vec<Rule> {

        rules
            .iter()
            .filter(|r| {

                if r.languages.is_empty() {
                    return true;
                }

                r.languages.iter().any(|l| l == language)

            })
            .cloned()
            .collect()
    }
}
