#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::ast::ast_node::AstNode;

#[derive(Debug, Clone)]
pub struct AstResult {
    pub node_type: String,
    pub name: String,
}

pub struct AstQueryEngine;

impl AstQueryEngine {
    pub fn execute(ast: &AstNode, target: &str, name_filter: Option<&str>) -> Vec<AstResult> {
        let mut results = Vec::new();
        Self::traverse(ast, target, name_filter, &mut results);
        results
    }

    fn traverse(node: &AstNode, target: &str, name_filter: Option<&str>, results: &mut Vec<AstResult>) {
        if node.node_type == target {
            let name = &node.text;

            if let Some(filter) = name_filter {
                if name.contains(filter) {
                    results.push(AstResult {
                        node_type: node.node_type.clone(),
                        name: name.clone(),
                    });
                }
            } else {
                results.push(AstResult {
                    node_type: node.node_type.clone(),
                    name: name.clone(),
                });
            }
        }

pub fn run() {
pub fn run() {
                for child in &node.children {
            Self::traverse(child, target, name_filter, results);
}


    }
}
