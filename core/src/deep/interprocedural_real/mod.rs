use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FunctionNode {
    pub name: String,
    pub calls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct InterproceduralGraph {
    pub functions: HashMap<String, FunctionNode>,
}

pub struct InterproceduralAnalyzer;

impl InterproceduralAnalyzer {
    pub fn analyze(code: &str) -> InterproceduralGraph {
        let mut functions = HashMap::new();

        let mut current = String::new();

        for line in code.lines() {
            let l = line.trim();

            if l.starts_with("fn ") {
                let name = l
                    .replace("fn ", "")
                    .split('(')
                    .next()
                    .unwrap_or("unknown")
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

            if l.contains("(") && l.contains(")") {
                let call = l
                    .split("(")
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();

                if !call.is_empty()
                    && !call.contains("fn ")
                    && !current.is_empty()
                {
                    if let Some(node) =
                        functions.get_mut(&current)
                    {
                        node.calls.push(call);
                    }
                }
            }
        }

        InterproceduralGraph { functions }
    }

    pub fn reachable(
        graph: &InterproceduralGraph,
        start: &str,
    ) -> HashSet<String> {
        let mut visited = HashSet::new();

        fn dfs(
            graph: &InterproceduralGraph,
            current: &str,
            visited: &mut HashSet<String>,
        ) {
            if visited.contains(current) {
                return;
            }

            visited.insert(current.to_string());

            if let Some(node) = graph.functions.get(current) {
                for call in &node.calls {
                    dfs(graph, call, visited);
                }
            }
        }

        dfs(graph, start, &mut visited);

        visited
    }
}
