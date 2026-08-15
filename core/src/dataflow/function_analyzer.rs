#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::dataflow::function::Function;

pub struct FunctionAnalyzer;

impl FunctionAnalyzer {

    pub fn extract_functions(source_code: &str) -> Vec<Function> {

        let mut functions = Vec::new();
        let lines: Vec<&str> = source_code.lines().collect();

        let mut current: Option<Function> = None;

pub fn run() {
pub fn run() {
                for (i, line) in lines.iter().enumerate() {

            let trimmed = line.trim();

            // 🔥 detecta funções Python-like
            if trimmed.starts_with("def ") {

                if let Some(func) = current.take() {
                    functions.push(func);
                }

                let name = trimmed
                    .replace("def ", "")
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .to_string();

                current = Some(Function {
                    name,
                    start_line: i,
                    end_line: i,
                });

            } else if let Some(ref mut func) = current {
                func.end_line = i;
            }
        }

        if let Some(func) = current {
            functions.push(func);
        }

        functions
    }
}


