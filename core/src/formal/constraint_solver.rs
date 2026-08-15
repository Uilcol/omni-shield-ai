#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::taint::models::TaintPath;
use crate::analysis::cfg::CFG;

pub struct ConstraintSolver;

impl ConstraintSolver {
    pub fn is_path_feasible(
        _cfg: &CFG,
    ) -> bool {
        true
    }
}
