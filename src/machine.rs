use std::io::{Read, Write};

use crate::instruction::{Instruction, InstructionType};

pub struct Machine<'a> {
    code: &'a Vec<Instruction>,
    ip: usize,
    memory: [i8; 30000],
    dp: usize,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    buf: Vec<u8>,
}

impl<'a> Machine<'a> {
    pub fn new(code: &'a Vec<Instruction>, input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
        return Self {
            code,
            ip: 0,
            memory: [0; 30000],
            dp: 0,
            input,
            output,
            buf: vec![0],
        };
    }

    pub fn execute(&mut self) {
        while self.ip < self.code.len() {
            if let Some(instruction) = self.code.get(self.ip) {
                match instruction.ins_type {
                    InstructionType::Plus => self.memory[self.dp] += instruction.argument,
                    InstructionType::Minus => self.memory[self.dp] -= instruction.argument,
                    InstructionType::Right => self.dp += instruction.argument as usize,
                    InstructionType::Left => self.dp -= instruction.argument as usize,
                    InstructionType::ReadChar => {
                        for _ in 0..instruction.argument {
                            self.read_char();
                        }
                    }
                    InstructionType::PutChar => {
                        for _ in 0..instruction.argument {
                            self.put_char();
                        }
                    }
                    InstructionType::JumpIfZero => {
                        if self.memory[self.dp] == 0 {
                            self.ip = instruction.argument as usize;
                        }
                    }
                    InstructionType::JumpIfNotZero => {
                        if self.memory[self.dp] != 0 {
                            self.ip = instruction.argument as usize;
                        }
                    }
                }
            }
            self.ip += 1;
        }
    }

    fn read_char(&mut self) {
        match self.input.read(&mut self.buf) {
            Ok(n) => {
                if n != 1 {
                    panic!("wrong num bytes read");
                }

                self.memory[self.dp] = self.buf[0] as i8;
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn put_char(&mut self) {
        self.buf[0] = self.memory[self.dp] as u8;

        match self.output.write(&self.buf) {
            Ok(n) => {
                if n != 1 {
                    panic!("wrong num bytes written");
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
