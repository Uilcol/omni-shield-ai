use crate::risk::scoring::RiskScore;

pub struct RiskPrioritizer;

impl RiskPrioritizer {
    pub fn prioritize(mut items: Vec<RiskScore>) -> Vec<RiskScore> {
        items.sort_by(|a, b| {
            b.final_score.cmp(&a.final_score)
        });

        items
    }
}
