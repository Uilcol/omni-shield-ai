#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct AnalysisMetrics {
    pub files_scanned: usize,

    pub findings_detected: usize,

    pub start_time: Instant,

    pub duration: Option<Duration>,
}

impl AnalysisMetrics {
    pub fn new() -> Self {
        Self {
            files_scanned: 0,

            findings_detected: 0,

            start_time: Instant::now(),

            duration: None,
        }
    }

    pub fn file_scanned(&mut self) {
        self.files_scanned += 1;
    }

    pub fn findings_added(&mut self, count: usize) {
        self.findings_detected += count;
    }

    pub fn finish(&mut self) {
        self.duration = Some(self.start_time.elapsed());
    }
}
