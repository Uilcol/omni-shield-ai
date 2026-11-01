use crate::benchmark::dataset_loader::DatasetLoader;
use crate::benchmark::report::BenchmarkReport;
use crate::runtime::executor::RuntimeExecutor;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn run_benchmark() -> Result<BenchmarkReport, io::Error> {
    let dataset_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../benchmark/dataset/basic_dataset.json"
    );

    let samples = DatasetLoader::load_labeled_from_file(dataset_path)?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| {
            io::Error::other(format!(
                "Unable to determine benchmark timestamp: {}",
                error
            ))
        })?
        .as_nanos();

    let benchmark_root = std::env::temp_dir().join(format!(
        "omnishield-benchmark-{}-{}",
        std::process::id(),
        timestamp
    ));

    fs::create_dir_all(&benchmark_root)?;

    let result = run_samples(&samples, &benchmark_root);

    let cleanup_result = fs::remove_dir_all(&benchmark_root);

    match (result, cleanup_result) {
        (Ok(report), Ok(())) => Ok(report),
        (Ok(report), Err(error)) => {
            eprintln!(
                "Warning: failed to remove benchmark directory '{}': {}",
                benchmark_root.display(),
                error
            );
            Ok(report)
        }
        (Err(error), Ok(())) => Err(error),
        (Err(error), Err(cleanup_error)) => Err(io::Error::other(format!(
            "{}; additionally failed to clean benchmark directory '{}': {}",
            error,
            benchmark_root.display(),
            cleanup_error
        ))),
    }
}

fn run_samples(
    samples: &[crate::benchmark::dataset_loader::DatasetSample],
    benchmark_root: &PathBuf,
) -> Result<BenchmarkReport, io::Error> {
    let started = std::time::Instant::now();

    let mut true_positive = 0usize;
    let mut true_negative = 0usize;
    let mut false_positive = 0usize;
    let mut false_negative = 0usize;

    for (index, sample) in samples.iter().enumerate() {
        let sample_path = benchmark_root.join(format!("sample_{:04}.py", index + 1));

        fs::write(&sample_path, &sample.code)?;

        let findings = RuntimeExecutor::execute(
            sample_path
                .to_str()
                .ok_or_else(|| io::Error::other("Invalid benchmark sample path"))?,
        );

        let detected = !findings.is_empty();

        match (sample.vulnerable, detected) {
            (true, true) => true_positive += 1,
            (false, false) => true_negative += 1,
            (false, true) => false_positive += 1,
            (true, false) => false_negative += 1,
        }
    }

    let latency_ms = started.elapsed().as_millis();

    let positive_predictions = true_positive + false_positive;
    let actual_positives = true_positive + false_negative;
    let actual_negatives = true_negative + false_positive;

    let precision = if positive_predictions == 0 {
        0.0
    } else {
        true_positive as f64 / positive_predictions as f64
    };

    let recall = if actual_positives == 0 {
        0.0
    } else {
        true_positive as f64 / actual_positives as f64
    };

    let fp_rate = if actual_negatives == 0 {
        0.0
    } else {
        false_positive as f64 / actual_negatives as f64
    };

    Ok(BenchmarkReport {
        total_samples: samples.len(),
        true_positive,
        false_positive,
        true_negative,
        false_negative,
        fp_rate,
        precision,
        recall,
        latency_ms,
    })
}
