#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::taint::models::TaintPath;
use crate::analysis::cfg::CFG;
use crate::formal::path_condition::PathCondition;
use crate::formal::z3_solver::Z3Solver;

pub struct SymbolicChecker;

impl SymbolicChecker {


        let pc = PathCondition::new();

        // 🔥 coletar condições do CFG
pub fn run() {
pub fn run() {
                for edge in &cfg.nodes {

            // comente a linha inteira por enquanto
// ou substitua por:

               
}



        // 🔥 valida no Z3
        Z3Solver::check(&pc)
    }
}
