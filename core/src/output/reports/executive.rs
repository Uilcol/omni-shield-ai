#[derive(Debug, Clone)]
pub struct ExecutiveSummary {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

pub struct ExecutiveReport;

impl ExecutiveReport {
    pub fn render(summary: &ExecutiveSummary) -> String {
        format!(
r#"
==============================
 OMNIUIL AI ENTERPRISE REPORT
==============================

Critical Findings : {}
High Findings     : {}
Medium Findings   : {}
Low Findings      : {}

Risk Score: {}

"#,
            summary.critical,
            summary.high,
            summary.medium,
            summary.low,
            summary.critical * 10
                + summary.high * 7
                + summary.medium * 4
                + summary.low
        )
    }
}
