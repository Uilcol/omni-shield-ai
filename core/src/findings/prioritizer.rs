use crate::findings::finding::Finding;

pub struct Prioritizer;

impl Prioritizer {
    pub fn sort(mut findings: Vec<Finding>) -> Vec<Finding> {
        findings.sort_by(|a, b| {
            Self::severity_score(&b.severity)
                .cmp(&Self::severity_score(&a.severity))
                .then_with(|| {
                    b.confidence
                        .partial_cmp(&a.confidence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        findings
    }

    fn severity_score(severity: &str) -> u8 {
        match severity.to_lowercase().as_str() {
            "critical" => 5,
            "high" => 4,
            "medium" => 3,
            "low" => 2,
            "info" => 1,
            _ => 0,
        }
    }
}
