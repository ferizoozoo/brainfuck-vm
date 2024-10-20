use std::io::{Read, Write};

pub struct Machine {
    code: String,
    ip: usize,
    memory: [i8; 30000],
    dp: usize,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    buf: Vec<u8>,
}

impl Machine {
    pub fn new(code: String, input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
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
            if let Some(instruction) = self.code.chars().nth(self.ip) {
                match instruction {
                    '+' => self.memory[self.dp] += 1,
                    '-' => self.memory[self.dp] -= 1,
                    '>' => self.dp += 1,
                    '<' => self.dp -= 1,
                    ',' => self.read_char(),
                    '.' => self.put_char(),
                    '[' => {
                        if self.memory[self.dp] == 0 {
                            let mut depth = 1;

                            while depth != 0 {
                                self.ip += 1;

                                if let Some(ch) = self.code.chars().nth(self.ip) {
                                    match ch {
                                        '[' => depth += 1,
                                        ']' => depth -= 1,
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                    ']' => {
                        if self.memory[self.dp] != 0 {
                            let mut depth = 1;

                            while depth != 0 {
                                self.ip -= 1;

                                if let Some(ch) = self.code.chars().nth(self.ip) {
                                    match ch {
                                        '[' => depth -= 1,
                                        ']' => depth += 1,
                                        _ => (),
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }

                self.ip += 1;
            }
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
