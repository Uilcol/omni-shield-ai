#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub pattern: String,
}

pub struct RuleParser;

impl RuleParser {
    pub fn parse(content: &str) -> Vec<Rule> {
        content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| Rule {
                id: line.to_string(),
                pattern: line.to_string(),
            })
            .collect()
    }
}
