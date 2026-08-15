#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
// core/src/language/adapter.rs

use crate::finding::Finding;
use crate::rules::registry::RuleRegistry;

pub trait LanguageAdapter {
    /// Nome da linguagem (ex: "rust")
    fn name(&self) -> &'static str;

    /// Executa análise completa de um arquivo
    fn analyze(
        &self,
        source: &str,
        registry: &RuleRegistry,
    ) -> Vec<Finding>;
}
