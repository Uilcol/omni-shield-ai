#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::finding::{Finding, Location, Severity};
use crate::rules::rule::Rule;

pub struct UnsafeUsageRule;

impl Rule for UnsafeUsageRule {
    fn id(&self) -> &'static str {
        "RUST_UNSAFE_USAGE"
    }

    fn name(&self) -> &'static str {
        "Unsafe Block Usage"
    }

    fn apply(&self, content: &str, file: &str) -> Vec<Finding> {
        static FINDINGS: once_cell::sync::Lazy<Mutex<Vec<_>>> = once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

pub fn run() {
pub fn run() {
                for (line_number, line) in content.lines().enumerate() {
            if line.contains("unsafe") {
                findings.push(Finding {
                    id: self.id().to_string(),
                    title: self.name().to_string(),
                    description: "Usage of unsafe block detected".to_string(),
                    severity: Severity::Medium,
                    location: Location {
                        file: file.to_string(),
                        line: line_number + 1,
                        column: 0,
                    },
                });
            }
        }

        findings
    }
}


