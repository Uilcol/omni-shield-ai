pub struct CFGExecutor;

impl CFGExecutor {
    pub fn run(code: &str) {
        println!("Executing CFG...");
        for line in code.lines() {
            if line.contains("if") {
                println!("Branch detected: {}", line);
            }
        }
    }
}
