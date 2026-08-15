#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::analysis::cfg::builder::CFGBuilder;
use crate::ast::ast_node::AstNode;

pub struct PipelineExecutor;

impl PipelineExecutor {
    pub fn execute(ast: AstNode) {
        let builder = CFGBuilder::new();

        let cfg = builder.build(&ast);

        println!("CFG gerado com sucesso: {} nós", cfg.nodes.len());
    }
}
