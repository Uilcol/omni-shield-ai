#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub output_file: String,
}

impl PipelineConfig {
    pub fn new(_project: &str, output: &str) -> Self {
        Self {
            output_file: output.to_string(),
        }
    }
}
