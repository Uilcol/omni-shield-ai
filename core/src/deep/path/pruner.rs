pub struct PathPruner;

impl PathPruner {
    pub fn should_prune(path_len: usize) -> bool {
        path_len > 20
    }
}
