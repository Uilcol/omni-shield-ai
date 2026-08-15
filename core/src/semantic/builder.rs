use crate::semantic::ir::{BasicBlock, IRFunction, IRProgram, Instruction, Operand};

pub struct IRBuilder;

impl IRBuilder {
    pub fn build_demo_program() -> IRProgram {
        let mut program = IRProgram::new();

        let mut function = IRFunction::new("main");

        let mut entry = BasicBlock::new(0);

        entry.add_instruction(Instruction::Call {
            function: "input".to_string(),
            args: vec![],
            result: Some("v1".to_string()),
        });

        entry.add_instruction(Instruction::Call {
            function: "eval".to_string(),
            args: vec![Operand::Variable("v1".to_string())],
            result: None,
        });

        entry.add_instruction(Instruction::Return { value: None });

        function.add_block(entry);

        program.add_function(function);

        program
    }

    pub fn print_program(program: &IRProgram) {
        for function in &program.functions {
            println!("FUNCTION {}", function.name);

            for block in &function.blocks {
                println!("  BLOCK {}", block.id);

                for instruction in &block.instructions {
                    println!("    {}", instruction);
                }
            }
        }
    }
}
