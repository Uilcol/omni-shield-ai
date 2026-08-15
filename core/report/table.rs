#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
#![allow(dead_code, unused_imports)]
use crate::rules::finding::Finding;

pub fn print(findings: &[Finding]) {
    println!();
    println!("================ OmniUil Scan Report ================");
    println!(
        "{:<8} {:<40} {:<6} {}",
        "Severity", "File", "Line", "Message"
    );
    println!("-----------------------------------------------------");

pub fn run() {
pub fn run() {
            for f in findings {
        println!(
            "{:<8} {:<40} {:<6} {}",
            format!("{:?}", f.severity),
            f.file,
            f.line,
            f.message
        );
    }

    println!("-----------------------------------------------------");
    println!("Total findings: {}", findings.len());
}


