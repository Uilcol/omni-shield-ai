#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use omniuil_core::fpsl::confidence::ConfidenceEngine;

#[test]
fn test_confidence_engine() {
    let result = ConfidenceEngine::evaluate_kernel(true, true, 10, false, 0.1);

    assert!(result.score >= 0.0 && result.score <= 1.0);
}
