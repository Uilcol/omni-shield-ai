#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use tree_sitter::Node;

use crate::ir::IRInstruction;

pub struct IRBuilder;

impl IRBuilder {

    pub fn build(root: Node, source: &str) -> Vec<IRInstruction> {

        let mut instructions = Vec::new();

        fn walk(node: Node, source: &str, out: &mut Vec<IRInstruction>) {

            let code = node.utf8_text(source.as_bytes()).unwrap_or("").to_string();

            // 🔥 INPUT
            if code.contains("input") {
                out.push(IRInstruction::Input {
                    variable: "var".to_string(),
                });
            }

            // 🔥 SANITIZE
            if code.contains("sanitize") {
                out.push(IRInstruction::Sanitize {
                    variable: "var".to_string(),
                });
            }

            // 🔥 CALL (eval, exec, etc)
            if code.contains("eval") || code.contains("exec") {
                out.push(IRInstruction::Call {
                    function: "eval".to_string(),
                    args: vec!["var".to_string()],
                });
            }

            // 🔥 IF
            if node.kind().contains("if") {
                out.push(IRInstruction::Condition {
                    expr: code.clone(),
                });
            }

            let mut cursor = node.walk();
pub fn run() {
pub fn run() {
                    for child in node.children(&mut cursor) {
                walk(child, source, out);
            }
        }

        walk(root, source, &mut instructions);

        instructions
    }
}


