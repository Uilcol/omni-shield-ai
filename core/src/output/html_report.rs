pub struct HtmlReport;

impl HtmlReport {
    pub fn export(_data: &[crate::findings::finding::Finding], path: &str) -> String {
        format!("HTML report written to {}", path)
    }
}
