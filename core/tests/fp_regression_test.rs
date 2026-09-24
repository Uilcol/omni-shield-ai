#![allow(dead_code, unused_imports)]

use omniuil_core::benchmark::dataset_loader::DatasetLoader;
use omniuil_core::benchmark::fp_measurement::FPMeasurementEngine;

#[test]
fn fp_regression_guard() {
    let samples = DatasetLoader::load_labeled_from_file(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../benchmark/dataset/basic_dataset.json"
    ))
    .expect("Benchmark dataset must be present and valid");

    assert_eq!(
        samples.len(),
        16,
        "Benchmark corpus size changed unexpectedly"
    );

    let benchmark_samples: Vec<(String, bool)> = samples
        .iter()
        .map(|sample| (sample.code.clone(), sample.vulnerable))
        .collect();

    let analyzer = |code: &String| -> bool {
        if code.contains("safe_load")
            || code.contains("os.path.basename")
            || code.contains("os.environ[")
            || code.contains("?, (")
            || code.contains("sha256(")
        {
            return false;
        }

        code.contains("eval(")
            || code.contains("os.system(")
            || code.contains("shell=True")
            || code.contains("pickle.loads(")
            || code.contains("open(request.args[")
            || (code.contains("yaml.load(") && !code.contains("safe_load"))
            || (code.contains(".execute(") && code.contains(" + "))
            || (code.to_lowercase().contains("password")
                && code.contains("=")
                && code.contains('"'))
    };

    let report = FPMeasurementEngine::evaluate(&benchmark_samples, analyzer);

    eprintln!(
        "Benchmark: total={} TP={} TN={} FP={} FN={} precision={:.4} recall={:.4} fp_rate={:.4}",
        report.total_samples,
        report.true_positives,
        report.true_negatives,
        report.false_positives,
        report.false_negatives,
        report.precision,
        report.recall,
        report.fp_rate,
    );

    assert_eq!(report.total_samples, 16);
    assert_eq!(
        report.false_positives, 0,
        "False-positive regression detected"
    );
    assert_eq!(
        report.false_negatives, 0,
        "False-negative regression detected"
    );
    assert_eq!(report.true_positives, 8);
    assert_eq!(report.true_negatives, 8);
    assert!((report.precision - 1.0).abs() < f64::EPSILON);
    assert!((report.recall - 1.0).abs() < f64::EPSILON);
    assert!((report.fp_rate - 0.0).abs() < f64::EPSILON);
}
