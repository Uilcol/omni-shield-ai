#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use super::pipeline_config::PipelineConfig;

pub struct CIGenerator;

impl CIGenerator {
    pub fn generate_github_actions(config: &PipelineConfig) -> String {
        format!(
            r#"
name: OmniUil Security Scan

on: [push, pull_request]

jobs:
  security:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Run OmniUil Scanner
        run: omniuil scan {} --sarif {}
"#,
            config.output_file, config.output_file
        )
    }
}
