#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub code: String,
    pub predecessors: Vec<usize>,
    pub successors: Vec<usize>,
}
