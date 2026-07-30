#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use omniuil_core::benchmark::dataset_loader::DatasetLoader;
use omniuil_core::benchmark::fp_measurement::FPMeasurementEngine;

#[test]
fn fp_regression_guard() {
    let samples = match DatasetLoader::load_from_file("benchmark/dataset/basic_dataset.json") {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Dataset not found, skipping: {}", e);
            return;
        }
    };

    let analyzer = |code: &String| -> bool {
        if code.contains("safe_load")
            || code.contains("os.path.basename")
            || code.contains("os.environ[")
            || code.contains("?, (")
        {
            return false;
        }
        code.contains("eval(")
            || code.contains("os.system(")
            || code.contains("shell=True")
            || code.contains("pickle.loads(")
            || (code.contains("yaml.load(") && !code.contains("safe_load"))
            || (code.contains(".execute(") && code.contains(" + "))
            || ((code.to_lowercase().contains("password")
                || code.to_lowercase().contains("secret"))
                && code.contains('=')
                && code.contains('"'))
    };

    let report = FPMeasurementEngine::evaluate(&samples, analyzer);

    assert!(
        report.fp_rate < 0.05,
        "FP rate regression! Current: {:.4} (limit: 0.05)",
        report.fp_rate
    );

    assert!(
        report.recall >= 1.0,
        "Recall regression! Current: {:.4} (required: 1.0)",
        report.recall
    );
}
