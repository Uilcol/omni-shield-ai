#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
#[allow(dead_code, unused_imports)]
use crate::finding::{Finding, Location, Severity};
use crate::rules::rule::Rule;

use syn::{visit::Visit, ExprMacro};

pub struct PanicRule;

impl Rule for PanicRule {
    fn id(&self) -> &'static str {
        "RUST-PANIC-001"
    }

    fn name(&self) -> &'static str {
        "Usage of panic! macro"
    }

    fn description(&self) -> &'static str {
        "panic!() causes immediate program termination. Avoid in production code."
    }

    fn severity(&self) -> Severity {
        Severity::High
    }

        let mut visitor = PanicVisitor {
            findings: Vec::new(),
        };

        visitor.visit_file(file);
        visitor.findings
    }
}

struct PanicVisitor {
    findings: Vec<Finding>,
}

impl<'ast> Visit<'ast> for PanicVisitor {
    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        if node.mac.path.is_ident("panic") {
            let location = Location {
                file: self.path.clone(),
                line: node.mac.span().start().line,
            };

            self.findings.push(Finding {
                id: "RUST-PANIC-001".to_string(),
                title: "Usage of panic! macro".to_string(),
                description:
                    "panic!() causes immediate program termination. Avoid in production code."
                        .to_string(),
                severity: Severity::High,
                location,
            });
        }

        syn::visit::visit_expr_macro(self, node);
    }
}
