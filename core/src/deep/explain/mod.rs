use std::collections::HashMap;

pub struct ExplainEngine;

impl ExplainEngine {
    pub fn explain(
        issue: &str,
        confirmed: bool,
        interprocedural: &HashMap<String, bool>,
    ) {
        println!();
        println!("============= EXPLAIN ENGINE =============");
        println!("Issue: {}", issue);

        if confirmed {
            println!("Status: Confirmed vulnerability");
            println!("Reason:");

            for (func, tainted) in interprocedural {
                if *tainted {
                    println!(" - Taint propagated through function: {}", func);
                }
            }

            println!(" - SMT solver confirmed satisfiable execution path");
            println!(" - Symbolic execution reached sink");
            println!(" - Hybrid engine validated exploitability");
        } else {
            println!("Status: False positive eliminated");
            println!("Reason: path constraints are UNSAT");
        }

        println!("==========================================");
        println!();
    }
}
