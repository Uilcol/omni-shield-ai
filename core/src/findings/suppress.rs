use crate::findings::finding::Finding;

pub struct SuppressionEngine;

impl SuppressionEngine {
    pub fn filter(findings: Vec<Finding>) -> Vec<Finding> {
        findings
            .into_iter()
            .filter(|f| {
                let file = f.file.to_lowercase();

                // suprime apenas arquivos artificiais de teste
                !(file.contains("test") || file.contains("demo") || file.contains("example"))
            })
            .collect()
    }
}
