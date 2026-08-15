use omniuil_core::semantic::builder::IRBuilder;

pub fn run_semantic_demo() {
    println!();
    println!("===============================");
    println!(" OmniShield AI Semantic Engine ");
    println!("===============================");
    println!();

    let program = IRBuilder::build_demo_program();

    IRBuilder::print_program(&program);

    println!();
    println!("[OK] Semantic IR generated successfully.");
}
