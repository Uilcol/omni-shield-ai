#[derive(Debug, Clone)]
pub struct OptimizedQuery {
    pub original: String,
    pub optimized: String,
}

pub struct QueryOptimizer;

impl QueryOptimizer {
    pub fn optimize(query: &str) -> OptimizedQuery {
        let mut optimized = query.replace("  ", " ");

        optimized = optimized.replace("\n", " ");

        optimized = optimized.trim().to_string();

        OptimizedQuery {
            original: query.to_string(),
            optimized,
        }
    }
}
