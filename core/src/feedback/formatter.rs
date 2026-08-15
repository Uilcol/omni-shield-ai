#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct DeveloperFeedback {
    pub title: String,
    pub message: String,
    pub severity: String,
}

pub struct FeedbackFormatter;

impl FeedbackFormatter {
    /// Format security finding into developer friendly message
    pub fn format(rule_id: &str, explanation: &str, severity_score: f64) -> DeveloperFeedback {
        let severity = if severity_score > 0.8 {
            "HIGH"
        } else if severity_score > 0.5 {
            "MEDIUM"
        } else {
            "LOW"
        };

        let title = format!("Security Finding [{}]", rule_id);

        let message = format!(
            "{}\n\nWhy this matters:\n{}\n\nSuggested action: Review code flow and sanitize inputs if necessary.",
            explanation,
            explanation
        );

        DeveloperFeedback {
            title,
            message,
            severity: severity.to_string(),
        }
    }
}
