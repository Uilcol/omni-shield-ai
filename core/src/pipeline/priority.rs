#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct PipelinePriorityEngine;

impl PipelinePriorityEngine {
    pub fn compute_priority(risk_score: f64, confidence_score: f64) -> u8 {
        let priority = risk_score * 0.7 + confidence_score * 0.3;

        if priority > 8.0 {
            1
        } else if priority > 5.0 {
            2
        } else {
            3
        }
    }
}
