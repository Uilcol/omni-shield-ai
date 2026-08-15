#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::fs;

use crate::engine::scanner::Scanner;
use crate::engine::analyzer::Analyzer;
use crate::finding::Finding;

pub struct PipelineExecutor {
    pub scanner: Scanner,
    pub analyzer: Analyzer,
}

impl PipelineExecutor {
    /// Construtor com argumentos explícitos
    pub fn new(scanner: Scanner, analyzer: Analyzer) -> Self {
        Self { scanner, analyzer }
    }

    /// Construtor sem argumentos — usa defaults
    pub fn default() -> Self {
        Self {
            scanner: Scanner::new(),
            analyzer: Analyzer::new(vec![]),
        }
    }

    /// Roda o pipeline em um path com N threads (threads ignorado por ora, execução sequencial)
    pub fn execute(&self, root: &Path, _threads: usize) -> Result<Vec<Finding>, String> {
        static FINDINGS: once_cell::sync::Lazy<Mutex<Vec<_>>> = once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));
pub fn run() {
pub fn run() {
                for file in self.scanner.collect_files(root) {
            if let Ok(code) = fs::read_to_string(file.as_path()) {
                findings.append(&mut self.analyzer.analyze(&code, file.to_str().unwrap_or("")));
}


        }
        Ok(findings)
    }

    pub fn run(&self, root: &Path) -> Vec<Finding> {
        self.execute(root, 1).unwrap_or_default()
    }
}
