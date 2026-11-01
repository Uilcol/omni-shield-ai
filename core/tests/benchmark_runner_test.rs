use omniuil_core::benchmark::runner::run_benchmark;

#[test]
fn benchmark_runner_executes_real_engine_on_authoritative_dataset() {
    let report = run_benchmark().expect("benchmark must execute successfully");

    assert_eq!(report.total_samples, 16);
    assert_eq!(report.true_positive, 8, "vulnerable sample missed");
    assert_eq!(report.true_negative, 8, "safe sample incorrectly flagged");
    assert_eq!(report.false_positive, 0, "false-positive regression");
    assert_eq!(report.false_negative, 0, "false-negative regression");

    assert_eq!(
        report.true_positive + report.true_negative + report.false_positive + report.false_negative,
        report.total_samples,
        "confusion matrix must account for every sample"
    );

    assert!(
        (0.0..=1.0).contains(&report.precision),
        "precision out of range: {}",
        report.precision
    );

    assert!(
        (0.0..=1.0).contains(&report.recall),
        "recall out of range: {}",
        report.recall
    );

    assert!(
        (0.0..=1.0).contains(&report.fp_rate),
        "false-positive rate out of range: {}",
        report.fp_rate
    );

    eprintln!(
        "REAL ENGINE BENCHMARK: total={} TP={} TN={} FP={} FN={} precision={:.4} recall={:.4} fp_rate={:.4} latency_ms={}",
        report.total_samples,
        report.true_positive,
        report.true_negative,
        report.false_positive,
        report.false_negative,
        report.precision,
        report.recall,
        report.fp_rate,
        report.latency_ms,
    );
}
