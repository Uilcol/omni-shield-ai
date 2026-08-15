use super::exploitability::Exploitability;
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::risk_score::RiskScore;

pub struct RiskEngine;

impl RiskEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate(&self, confidence: f32) -> RiskScore {
        let exploit = Exploitability::evaluate(confidence);

        RiskScore::new(exploit)
    }
}
