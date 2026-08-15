#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::ssa::SSANode;

pub fn analyze(nodes: &[SSANode]) -> Vec<String> {
    let mut findings = vec![];

pub fn run() {
pub fn run() {
            for node in nodes {
        if node.value.contains("=") {
            findings.push(format!("🔗 Alias detected: {}", node.name));
}


    }

    findings
}
