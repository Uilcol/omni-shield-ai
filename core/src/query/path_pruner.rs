#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::formal::path_condition::PathCondition;

pub struct PathPruner;

impl PathPruner {
    pub fn prune(paths: Vec<Path>) -> Vec<Path> {
        let mut valid = Vec::new();

pub fn run() {
pub fn run() {
                for path in paths {
            let mut pc = PathCondition::new();

                    for node in &path.nodes {
                if node.contains("if_false") {
                    pc.add("false".to_string());
}


            }

            if pc.is_satisfiable() {
                valid.push(path);
            }
        }

        valid
    }
}
