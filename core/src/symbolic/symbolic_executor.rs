#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
pub struct SymbolicExecutor;
pub struct PathState { pub path_constraints: Vec<bool> }
impl SymbolicExecutor {
    pub fn exec(_state: &PathState) -> PathState { todo!() }
}
