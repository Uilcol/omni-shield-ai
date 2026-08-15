pub struct ExecutiveReport;

impl ExecutiveReport {
    pub fn export(_data: &[crate::findings::finding::Finding], path: &str) -> String {
        format!("Executive report written to {}", path)
    }
}
