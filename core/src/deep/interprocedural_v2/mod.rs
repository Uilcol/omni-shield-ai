use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionSummary {
    pub name: String,
    pub parameters_tainted: bool,
    pub returns_tainted: bool,
    pub calls: Vec<String>,
}

pub struct InterproceduralEngine;

impl InterproceduralEngine {
    pub fn analyze(items: Vec<FunctionSummary>) -> HashMap<String, bool> {
        let mut result = HashMap::new();
        let mut graph = HashMap::<String, Vec<String>>::new();

        for item in &items {
            graph.insert(item.name.clone(), item.calls.clone());
            result.insert(
                item.name.clone(),
                item.parameters_tainted || item.returns_tainted,
            );
        }

        let mut changed = true;

        while changed {
            changed = false;

            for (func, deps) in &graph {
                let mut tainted = *result.get(func).unwrap_or(&false);

                for dep in deps {
                    if *result.get(dep).unwrap_or(&false) {
                        tainted = true;
                    }
                }

                if result.get(func) != Some(&tainted) {
                    result.insert(func.clone(), tainted);
                    changed = true;
                }
            }
        }

        result
    }
}
