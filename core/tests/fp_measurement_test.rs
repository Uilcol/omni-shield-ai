#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use omniuil_core::benchmark::fp_measurement::FPMeasurementEngine;

#[test]
fn test_fp_measurement_engine() {
    let samples = vec![
        ("vulnerable_code_1".to_string(), true),
        ("safe_code_1".to_string(), false),
        ("vulnerable_code_2".to_string(), true),
        ("safe_code_2".to_string(), false),
    ];

    let analyzer = |code: &String| code.contains("vulnerable");

    let report = FPMeasurementEngine::evaluate(&samples, analyzer);

    assert_eq!(report.total_samples, 4);
    assert!(report.precision >= 0.0);
    assert!(report.recall >= 0.0);
}
