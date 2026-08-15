#[derive(Debug, Clone)]
pub struct RiskScore {
    pub issue: String,
    pub severity: u32,
    pub confidence: u32,
    pub exploitability: u32,
    pub business_impact: u32,
    pub final_score: u32,
}

pub struct RiskScorer;

impl RiskScorer {
    pub fn calculate(issue: &str) -> RiskScore {
        let severity = if issue.contains("SQL") { 10 } else { 7 };
        let confidence = 9;
        let exploitability = 8;
        let business_impact = 9;

        let final_score =
            (severity + confidence + exploitability + business_impact) / 4;

        RiskScore {
            issue: issue.to_string(),
            severity,
            confidence,
            exploitability,
            business_impact,
            final_score,
        }
    }
}
