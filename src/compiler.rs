use std::collections::VecDeque;

use crate::instruction::{Instruction, InstructionType};

pub struct Compiler {
    code: String,
    code_length: usize,
    position: usize,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new(code: String) -> Self {
        let code_length = code.len();

        Self {
            code,
            code_length,
            position: 0,
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self) -> &Vec<Instruction> {
        let mut loop_stack: VecDeque<usize> = VecDeque::new();

        while self.position < self.code_length {
            if let Some(current) = self.code.chars().nth(self.position) {
                match current {
                    '+' => self.compile_foldable_instruction('+', InstructionType::Plus),
                    '-' => self.compile_foldable_instruction('-', InstructionType::Minus),
                    '>' => self.compile_foldable_instruction('>', InstructionType::Right),
                    '<' => self.compile_foldable_instruction('<', InstructionType::Left),
                    '.' => self.compile_foldable_instruction('.', InstructionType::PutChar),
                    ',' => self.compile_foldable_instruction(',', InstructionType::ReadChar),
                    '[' => {
                        let ins_pos = self.emit_with_arg(InstructionType::JumpIfZero, 0);
                        loop_stack.push_front(ins_pos);
                    }
                    ']' => {
                        let open_instruction_pos = loop_stack.pop_front().unwrap();
                        let close_instruction_pos = self.emit_with_arg(
                            InstructionType::JumpIfNotZero,
                            open_instruction_pos as i8,
                        );
                        self.instructions[open_instruction_pos].argument =
                            close_instruction_pos as i8;
                    }
                    _ => (),
                }
            }
            self.position += 1
        }

        return &self.instructions;
    }

    fn compile_foldable_instruction(&mut self, ch: char, ins_type: InstructionType) {
        let mut count = 1;

        while self.position < self.code_length - 1
            && self.code.chars().nth(self.position + 1).unwrap() == ch
        {
            count += 1;
            self.position += 1;
        }

        self.emit_with_arg(ins_type, count);
    }

    fn emit_with_arg(&mut self, ins_type: InstructionType, arg: i8) -> usize {
        self.instructions.push(Instruction {
            ins_type,
            argument: arg,
        });
        return self.instructions.len() - 1;
    }
}
