#[derive(Debug, Clone)]
pub struct FunctionFlow {
    pub function_name: String,
    pub returns_tainted: bool,
}

pub struct InterproceduralAnalyzer;

impl InterproceduralAnalyzer {
    pub fn analyze(code_lines: Vec<String>) -> Vec<FunctionFlow> {
        let mut flows = Vec::new();

        let mut current_function = String::new();
        let mut inside_function = false;
        let mut tainted_return = false;

        for line in code_lines {
            let trimmed = line.trim();

            if trimmed.starts_with("def ") {
                if !current_function.is_empty() {
                    flows.push(FunctionFlow {
                        function_name: current_function.clone(),
                        returns_tainted: tainted_return,
                    });
                }

                current_function = trimmed
                    .replace("def ", "")
                    .split('(')
                    .next()
                    .unwrap_or("")
                    .to_string();

                inside_function = true;
                tainted_return = false;
            }

            if inside_function {
                if trimmed.contains("input(")
                    || trimmed.contains("request.GET")
                    || trimmed.contains("request.POST")
                    || trimmed.contains("user_input")
                {
                    tainted_return = true;
                }

                if trimmed.starts_with("return ") && tainted_return {
                    tainted_return = true;
                }
            }
        }

        if !current_function.is_empty() {
            flows.push(FunctionFlow {
                function_name: current_function,
                returns_tainted: tainted_return,
            });
        }

        flows
    }
}
