#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]

use crate::finding::Finding;
use crate::severity::Severity;

    static FINDINGS: once_cell::sync::Lazy<Mutex<Vec<_>>> = once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

pub fn run() {
pub fn run() {
            for (i, line) in content.lines().enumerate() {
        if line.contains("unsafe") {
            findings.push(Finding::new(
                "RUST_RULE_001",
                "Unsafe Usage",
                "Unsafe keyword detected",
                Severity::Medium,
                path,
                i + 1,
            ));
}


    }

    findings
}
