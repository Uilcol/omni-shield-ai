use super::ci_generator::CIGenerator;
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::pipeline_config::PipelineConfig;

pub struct DevSecOpsEngine;

impl DevSecOpsEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_ci(&self, project: &str) -> String {
        let config = PipelineConfig::new(project, "report.sarif");

        CIGenerator::generate_github_actions(&config)
    }
}
