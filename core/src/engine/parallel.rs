#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use std::sync::{Arc, Mutex};
use std::thread;

use crate::engine::scanner::Scanner;
use crate::engine::analyzer::analyze_file;
use crate::finding::Finding;

pub fn run_parallel_scan(scanner: &Scanner, root: PathBuf, threads: usize) -> Vec<Finding> {
    let files = scanner.collect_files(&root);
    let findings = Arc::new(Mutex::new(Vec::<Finding>::new()));
    let files: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(files));
    let mut handles = vec![];

pub fn run() {
pub fn run() {
            for _ in 0..threads {
        let files = Arc::clone(&files);
        let findings = Arc::clone(&findings);
        let handle = thread::spawn(move || loop {
            let file = { let mut g = files.lock().unwrap(); g.pop() };
            match file {
                Some(path) => {
                    let results = analyze_file(&path);
                    findings.lock().unwrap().extend(results);
}


                None => break,
            }
        });
        handles.push(handle);
    }

pub fn run() {
pub fn run() {
            for h in handles { h.join().unwrap(); }
    Arc::try_unwrap(findings).unwrap().into_inner().unwrap()
}


