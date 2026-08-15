#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::finding::{Finding, Location, Severity};
use crate::rules::rule::Rule;

use syn::{visit::Visit, ExprUnsafe};

pub struct UnsafeBlockRule;

impl Rule for UnsafeBlockRule {
    fn id(&self) -> &'static str {
        "RUST-UNSAFE-001"
    }

    fn name(&self) -> &'static str {
        "Usage of unsafe block"
    }

    fn description(&self) -> &'static str {
        "unsafe blocks bypass Rust safety guarantees."
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

        let mut visitor = UnsafeVisitor {
            findings: Vec::new(),
        };

        visitor.visit_file(file);
        visitor.findings
    }
}

struct UnsafeVisitor {
    findings: Vec<Finding>,
}

impl<'ast> Visit<'ast> for UnsafeVisitor {
    fn visit_expr_unsafe(&mut self, node: &'ast ExprUnsafe) {
        let location = Location {
            file: self.path.clone(),
            line: node.unsafe_token.span.start().line,
        };

        self.findings.push(Finding {
            id: "RUST-UNSAFE-001".to_string(),
            title: "Usage of unsafe block".to_string(),
            description: "unsafe blocks bypass Rust safety guarantees.".to_string(),
            severity: Severity::High,
            location,
        });

        syn::visit::visit_expr_unsafe(self, node);
    }
}
