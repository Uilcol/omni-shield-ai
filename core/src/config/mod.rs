#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::severity::Severity;

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub threshold: Severity,
    pub fail_on: Option<Severity>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            threshold: Severity::Low,
            fail_on: None,
        }
    }
}
