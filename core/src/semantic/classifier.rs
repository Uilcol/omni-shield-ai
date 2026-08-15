#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::vulnerability_taxonomy::VulnerabilityCategory;

pub struct SemanticClassifier;

impl SemanticClassifier {
    pub fn classify(rule_id: &str, flow_context: &str) -> VulnerabilityCategory {
        let mut category = VulnerabilityCategory::from_rule(rule_id);

        // Contextual semantic enhancement (MVP heuristic)

        if flow_context.contains("password") || flow_context.contains("token") {
            category = VulnerabilityCategory::DataLeak;
        }

        category
    }
}
