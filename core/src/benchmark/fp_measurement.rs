#[derive(Debug, Clone, PartialEq)]
pub struct FPMeasurementReport {
    pub total_samples: usize,
    pub true_positives: usize,
    pub true_negatives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
    pub precision: f64,
    pub recall: f64,
    pub fp_rate: f64,
}

pub trait BenchmarkSample {
    fn source_code(&self) -> &String;
    fn expected_vulnerable(&self) -> bool;
}

impl BenchmarkSample for String {
    fn source_code(&self) -> &String {
        self
    }

    fn expected_vulnerable(&self) -> bool {
        let code = self.as_str();

        code.contains("eval(")
            || code.contains("exec(")
            || code.contains("os.system")
            || code.contains("subprocess")
            || code.contains("SELECT ")
            || code.contains("INSERT ")
            || code.contains("UPDATE ")
            || code.contains("DELETE ")
            || code.contains("password")
            || code.contains("secret")
            || code.contains("MD5")
    }
}

impl BenchmarkSample for (String, bool) {
    fn source_code(&self) -> &String {
        &self.0
    }

    fn expected_vulnerable(&self) -> bool {
        self.1
    }
}

pub struct FPMeasurementEngine;

impl FPMeasurementEngine {
    pub fn evaluate<T, F>(samples: &Vec<T>, analyzer: F) -> FPMeasurementReport
    where
        T: BenchmarkSample,
        F: Fn(&String) -> bool,
    {
        let mut true_positives = 0usize;
        let mut true_negatives = 0usize;
        let mut false_positives = 0usize;
        let mut false_negatives = 0usize;

        for sample in samples {
            let expected_vulnerable = sample.expected_vulnerable();
            let detected = analyzer(sample.source_code());

            match (expected_vulnerable, detected) {
                (true, true) => true_positives += 1,
                (false, false) => true_negatives += 1,
                (false, true) => false_positives += 1,
                (true, false) => false_negatives += 1,
            }
        }

        let precision_denominator = true_positives + false_positives;

        let precision = if precision_denominator == 0 {
            0.0
        } else {
            true_positives as f64 / precision_denominator as f64
        };

        let recall_denominator = true_positives + false_negatives;

        let recall = if recall_denominator == 0 {
            0.0
        } else {
            true_positives as f64 / recall_denominator as f64
        };

        let negative_denominator = true_negatives + false_positives;

        let fp_rate = if negative_denominator == 0 {
            0.0
        } else {
            false_positives as f64 / negative_denominator as f64
        };

        FPMeasurementReport {
            total_samples: samples.len(),
            true_positives,
            true_negatives,
            false_positives,
            false_negatives,
            precision,
            recall,
            fp_rate,
        }
    }
}
