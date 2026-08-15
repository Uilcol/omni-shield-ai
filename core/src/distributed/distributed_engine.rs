#![allow(dead_code)]

pub struct DistributedScanEngine;

impl DistributedScanEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_paths(&self, paths: Vec<String>) {
        for path in paths {
            println!("Scanning: {}", path);
        }
    }
}
