#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct ValidationRunner;

impl ValidationRunner {

    pub fn validate_samples<F>(samples: &[String], analyzer: F) -> usize
    where
        F: Fn(&String) -> bool,
    {
        let mut detected = 0;

pub fn run() {
pub fn run() {
                for sample in samples {
            if analyzer(sample) {
                detected += 1;
            }
        }

        detected
    }
}


