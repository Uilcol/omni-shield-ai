use crate::analysis::cfg::CFG;
use crate::finding::Finding;

pub struct SecurityKernelFacade;

impl SecurityKernelFacade {
    pub fn analyze(_cfg: &CFG) -> Vec<Finding> {
        vec![]
    }
}
