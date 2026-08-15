use super::parser::Query;
use crate::analysis::ast::ASTNode;

pub struct QueryExecutor;

impl QueryExecutor {
    pub fn run(query: &Query, ast: &Vec<ASTNode>) {
        for node in ast {
            if format!("{:?}", node).contains(&query.pattern) {
                println!("Query match: {:?}", node);
            }
        }
    }
}
