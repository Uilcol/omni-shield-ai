use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityNodeKind {
    Source,
    Variable,
    Function,
    Call,
    Return,
    Sanitizer,
    Sink,
    Finding,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecurityNode {
    pub id: String,
    pub kind: SecurityNodeKind,
    pub label: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityEdgeKind {
    Defines,
    Assigns,
    Calls,
    Returns,
    FlowsTo,
    Sanitizes,
    Reaches,
    EvidenceFor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SecurityEdge {
    pub from: String,
    pub to: String,
    pub kind: SecurityEdgeKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityPathStep {
    pub node_id: String,
    pub kind: SecurityNodeKind,
    pub label: String,
    pub file: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecurityPath {
    pub steps: Vec<SecurityPathStep>,
}

impl SecurityPath {
    pub fn new(steps: Vec<SecurityPathStep>) -> Self {
        Self { steps }
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn source(&self) -> Option<&SecurityPathStep> {
        self.steps
            .iter()
            .find(|step| step.kind == SecurityNodeKind::Source)
    }

    pub fn sink(&self) -> Option<&SecurityPathStep> {
        self.steps
            .iter()
            .rev()
            .find(|step| step.kind == SecurityNodeKind::Sink)
    }

    pub fn files(&self) -> Vec<String> {
        let mut files = self
            .steps
            .iter()
            .map(|step| step.file.clone())
            .collect::<Vec<_>>();

        files.sort();
        files.dedup();
        files
    }

    pub fn line_span(&self) -> Option<(usize, usize)> {
        let first = self.steps.first()?;
        let last = self.steps.last()?;
        Some((first.line, last.line))
    }

    pub fn labels(&self) -> Vec<String> {
        self.steps.iter().map(|step| step.label.clone()).collect()
    }
}

#[derive(Debug, Default, Clone)]
pub struct SecurityGraph {
    nodes: HashMap<String, SecurityNode>,
    edges: Vec<SecurityEdge>,
    outgoing: HashMap<String, Vec<usize>>,
}

impl SecurityGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: SecurityNode) -> bool {
        let id = node.id.clone();

        if self.nodes.contains_key(&id) {
            return false;
        }

        self.nodes.insert(id, node);
        true
    }

    pub fn add_edge(&mut self, edge: SecurityEdge) -> bool {
        if !self.nodes.contains_key(&edge.from) || !self.nodes.contains_key(&edge.to) {
            return false;
        }

        if self.edges.iter().any(|existing| existing == &edge) {
            return false;
        }

        let index = self.edges.len();
        let from = edge.from.clone();

        self.edges.push(edge);
        self.outgoing.entry(from).or_default().push(index);

        true
    }

    pub fn node(&self, id: &str) -> Option<&SecurityNode> {
        self.nodes.get(id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &SecurityNode> {
        self.nodes.values()
    }

    pub fn edges(&self) -> &[SecurityEdge] {
        &self.edges
    }

    pub fn outgoing_edges(&self, node_id: &str) -> Vec<&SecurityEdge> {
        self.outgoing
            .get(node_id)
            .into_iter()
            .flatten()
            .filter_map(|index| self.edges.get(*index))
            .collect()
    }

    pub fn reachable_from(&self, start: &str) -> HashSet<String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        if !self.nodes.contains_key(start) {
            return visited;
        }

        visited.insert(start.to_string());
        queue.push_back(start.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(indices) = self.outgoing.get(&current) {
                for index in indices {
                    let Some(edge) = self.edges.get(*index) else {
                        continue;
                    };

                    if visited.insert(edge.to.clone()) {
                        queue.push_back(edge.to.clone());
                    }
                }
            }
        }

        visited
    }

    pub fn reaches_kind(&self, start: &str, target_kind: SecurityNodeKind) -> bool {
        self.reachable_from(start).into_iter().any(|id| {
            self.node(&id)
                .map(|node| node.kind == target_kind)
                .unwrap_or(false)
        })
    }

    pub fn security_paths(&self, source: &str, sink: &str, max_depth: usize) -> Vec<SecurityPath> {
        self.source_to_sink_paths(source, sink, max_depth)
            .into_iter()
            .filter_map(|ids| {
                let steps = ids
                    .into_iter()
                    .filter_map(|id| {
                        self.node(&id).map(|node| SecurityPathStep {
                            node_id: node.id.clone(),
                            kind: node.kind.clone(),
                            label: node.label.clone(),
                            file: node.file.clone(),
                            line: node.line,
                        })
                    })
                    .collect::<Vec<_>>();

                if steps.is_empty() {
                    None
                } else {
                    Some(SecurityPath::new(steps))
                }
            })
            .collect()
    }

    pub fn source_to_sink_paths(
        &self,
        source: &str,
        sink: &str,
        max_depth: usize,
    ) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut stack = vec![(source.to_string(), vec![source.to_string()])];

        while let Some((current, path)) = stack.pop() {
            if path.len() > max_depth + 1 {
                continue;
            }

            if current == sink {
                paths.push(path);
                continue;
            }

            for edge in self.outgoing_edges(&current) {
                if path.contains(&edge.to) {
                    continue;
                }

                let mut next_path = path.clone();
                next_path.push(edge.to.clone());
                stack.push((edge.to.clone(), next_path));
            }
        }

        paths.sort();
        paths
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
        self.outgoing.clear();
    }
}
