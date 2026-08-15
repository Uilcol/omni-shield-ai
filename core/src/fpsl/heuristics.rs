#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct HeuristicEngine;

impl HeuristicEngine {
    pub fn is_likely_false_positive(confidence_score: f64) -> bool {
        confidence_score < 0.4
    }
}
