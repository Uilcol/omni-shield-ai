pub struct SarifExporter;

impl SarifExporter {
    pub fn export(_data: &[crate::findings::finding::Finding], path: &str) -> String {
        format!("SARIF written to {}", path)
    }
}
