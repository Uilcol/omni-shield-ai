#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct RiskScore {
    pub score: f32,
    pub priority: String,
}

impl RiskScore {
    pub fn new(score: f32) -> Self {
        let priority = if score > 9.0 {
            "CRITICAL"
        } else if score > 7.0 {
            "HIGH"
        } else if score > 4.0 {
            "MEDIUM"
        } else {
            "LOW"
        };

        Self {
            score,
            priority: priority.to_string(),
        }
    }
}
