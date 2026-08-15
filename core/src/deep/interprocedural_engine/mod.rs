use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub calls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CallGraph {
    pub functions: HashMap<String, FunctionNode>,
}

pub struct InterproceduralEngine;

impl InterproceduralEngine {
    pub fn build(
        code: &str,
    ) -> CallGraph {
        let mut functions =
            HashMap::new();

        let mut current =
            String::new();

        for line in code.lines() {
            let l = line.trim();

            if l.starts_with("fn ") {
                let name = l
                    .replace("fn ", "")
                    .split("(")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                current = name.clone();

                functions.insert(
                    name.clone(),
                    FunctionNode {
                        name,
                        calls: Vec::new(),
                    },
                );
            }

            if l.contains("(")
                && l.contains(")")
                && !current.is_empty()
            {
                let target = l
                    .split("(")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if let Some(f) =
                    functions.get_mut(&current)
                {
                    if target != "if"
                        && target != "while"
                        && target != "for"
                        && target != current
                    {
                        f.calls.push(target);
                    }
                }
            }
        }

        CallGraph { functions }
    }

    pub fn reachable(
        graph: &CallGraph,
        start: &str,
    ) -> HashSet<String> {
        let mut visited =
            HashSet::new();

        Self::dfs(
            graph,
            start,
            &mut visited,
        );

        visited
    }

    fn dfs(
        graph: &CallGraph,
        current: &str,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(current) {
            return;
        }

        visited.insert(
            current.to_string(),
        );

        if let Some(node) =
            graph.functions.get(current)
        {
            for call in &node.calls {
                Self::dfs(
                    graph,
                    call,
                    visited,
                );
            }
        }
    }
}
