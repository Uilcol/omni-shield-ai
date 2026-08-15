#![allow(dead_code)]

use crate::symbolic::executor::SymbolicExecutor;
use crate::symbolic::vuln::VulnDetector;

pub struct SASTEngine;

impl SASTEngine {
    pub fn run() {
        println!("🔍 Running Symbolic Analysis...");

        let paths = SymbolicExecutor::execute();

        for (i, pc) in paths.iter().enumerate() {
            println!("Path {}: {:?}", i, pc);

            let vulns = VulnDetector::detect(pc);

            if !vulns.is_empty() {
                println!("⚠️ Vulnerabilities found:");
                for v in vulns {
                    println!("  - {}", v);
                }
            }
        }
    }
}
