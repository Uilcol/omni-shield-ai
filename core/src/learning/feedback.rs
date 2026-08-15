#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct UserFeedback {
    pub finding_id: String,
    pub is_false_positive: bool,
    pub confidence_override: Option<f64>,
}
