use crate::findings::finding::Finding;

pub struct RiskScoreEngine;

impl RiskScoreEngine {
    pub fn score(findings: &[Finding]) -> f64 {
        let mut score = 0.0;

        for f in findings {
            score += match f.severity.as_str() {
                "Critical" => 10.0,
                "High" => 5.0,
                "Medium" => 2.0,
                _ => 1.0,
            };
        }

        score
    }

    pub fn print(findings: &[Finding]) {
        let score = Self::score(findings);
        println!("Risk Score: {:.2}", score);
    }
}
