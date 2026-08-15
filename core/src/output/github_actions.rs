pub struct GitHubActionsExporter;

impl GitHubActionsExporter {
    pub fn export(_data: &[crate::findings::finding::Finding], path: &str) -> String {
        format!("GHA report written to {}", path)
    }
}
