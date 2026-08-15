#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use syn::File;

use crate::rules::RuleRegistry;
use crate::finding::Finding;

pub struct RustAdapter;

impl RustAdapter {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, code: &str) -> Result<File, syn::Error> {
         syn::parse_file(code)
    }

    pub fn analyze(
        &self,
        code: &str,
        registry: &RuleRegistry,
    ) -> Vec<Finding> {

        // tenta gerar AST (para futuras análises)
        let _ast = match self.parse(code) {
            Ok(ast) => ast,
            Err(_) => {
                return vec![];
            }
        };

        // executa regras baseadas no código

registry.run_for_language("rust", code, Path::new(file_path))
    }
}
