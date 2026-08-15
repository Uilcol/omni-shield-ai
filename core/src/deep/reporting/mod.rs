pub struct EnterpriseReporter;

impl EnterpriseReporter {
    pub fn print_banner() {
        println!();
        println!("==============================================");
        println!("      OMNIUIL AI — ENTERPRISE SECURITY");
        println!("   Taint + Symbolic + Z3 + Query DSL + SSA");
        println!("==============================================");
        println!();
    }

    pub fn final_result(vuln: bool) {
        if vuln {
            println!("RESULT: CONFIRMED VULNERABILITY");
        } else {
            println!("RESULT: FALSE POSITIVE ELIMINATED");
        }

        println!();
    }
}
