#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Variable(String),
    Literal(String),

    // 🔥 NOVO — SEMÂNTICA REAL
    Source(String),
    Sink(String),
    Sanitizer(String),
}

pub type NodeId = usize;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

impl Node {
    pub fn new(id: NodeId, kind: NodeKind) -> Self {
        Self { id, kind }
    }
}
