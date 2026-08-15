#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::model_state::AdaptiveModelState;

pub struct AdaptiveScoreEngine;

impl AdaptiveScoreEngine {
    pub fn adjust_confidence(base_score: f64, rule_id: &str, state: &AdaptiveModelState) -> f64 {
        let fp_rate = state.false_positive_rate(rule_id);

        let adjusted = base_score * (1.0 - fp_rate * 0.5);

        if adjusted < 0.0 {
            0.0
        } else if adjusted > 1.0 {
            1.0
        } else {
            adjusted
        }
    }
}
