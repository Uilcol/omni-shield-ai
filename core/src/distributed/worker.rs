#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::task::ScanTask;

pub struct Worker;

impl Worker {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, task: &ScanTask) {
        println!("Worker scanning: {}", task.path);

        // aqui chamaremos o PipelineExecutor no futuro
    }
}
