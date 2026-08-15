use std::fmt;

#[derive(Debug, Clone)]
pub enum Operand {
    Variable(String),
    Literal(String),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Assign {
        target: String,
        value: Operand,
    },

    Call {
        function: String,
        args: Vec<Operand>,
        result: Option<String>,
    },

    Return {
        value: Option<Operand>,
    },

    Branch {
        condition: Operand,
        true_block: usize,
        false_block: usize,
    },
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: usize,
    pub instructions: Vec<Instruction>,
}

impl BasicBlock {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[derive(Debug, Clone)]
pub struct IRFunction {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
}

impl IRFunction {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: BasicBlock) {
        self.blocks.push(block);
    }
}

#[derive(Debug, Clone)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

impl IRProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: IRFunction) {
        self.functions.push(function);
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Variable(v) => write!(f, "{v}"),
            Operand::Literal(v) => write!(f, "\"{v}\""),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Assign { target, value } => {
                write!(f, "{target} = {value}")
            }

            Instruction::Call {
                function,
                args,
                result,
            } => {
                let rendered_args = args
                    .iter()
                    .map(|a| format!("{a}"))
                    .collect::<Vec<String>>()
                    .join(", ");

                match result {
                    Some(r) => {
                        write!(f, "{r} = call {function}({rendered_args})")
                    }

                    None => {
                        write!(f, "call {function}({rendered_args})")
                    }
                }
            }

            Instruction::Return { value } => match value {
                Some(v) => write!(f, "return {v}"),
                None => write!(f, "return"),
            },

            Instruction::Branch {
                condition,
                true_block,
                false_block,
            } => {
                write!(
                    f,
                    "branch {condition} ? block_{true_block} : block_{false_block}"
                )
            }
        }
    }
}
