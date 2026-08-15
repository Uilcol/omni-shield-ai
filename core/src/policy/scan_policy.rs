#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::finding::Finding;
use crate::severity::Severity;

pub struct ScanPolicy {
    pub threshold: Severity,
    pub fail_on: Option<Severity>,
}

impl ScanPolicy {
    pub fn should_display(&self, finding: &Finding) -> bool {
        finding.severity >= self.threshold
    }

    pub fn should_fail(&self, findings: &[Finding]) -> bool {
        if let Some(fail_severity) = self.fail_on {
            findings.iter().any(|f| f.severity >= fail_severity)
        } else {
            false
        }
    }
}
