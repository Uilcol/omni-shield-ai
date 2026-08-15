#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct RuleGenerator;

impl RuleGenerator {
    pub fn generate(pattern: &str) -> String {
        format!("AUTO-GENERATED-RULE: detect pattern {}", pattern)
    }
}
