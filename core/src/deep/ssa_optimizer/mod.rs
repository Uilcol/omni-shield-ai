use std::collections::HashMap;

pub struct SSAOptimizer;

impl SSAOptimizer {
    pub fn optimize(
        ssa: &HashMap<
            usize,
            Vec<String>,
        >,
    ) -> HashMap<
        usize,
        Vec<String>,
    > {
        let mut optimized =
            HashMap::new();

        for (block, vars) in ssa {
            let mut unique =
                Vec::<String>::new();

            for v in vars {
                if !unique.contains(v) {
                    unique.push(v.clone());
                }
            }

            optimized.insert(
                *block,
                unique,
            );
        }

        optimized
    }
}
