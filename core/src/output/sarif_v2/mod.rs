#[derive(Debug, Clone)]
pub struct SarifExporterV2;

impl SarifExporterV2 {
    pub fn export(findings: usize) -> String {
        format!(
            r#"{{
  "version": "2.1.0",
  "runs": [
    {{
      "tool": {{
        "driver": {{
          "name": "OmniUil AI",
          "version": "0.3.0"
        }}
      }},
      "results_count": {}
    }}
  ]
}}"#,
            findings
        )
    }
}
