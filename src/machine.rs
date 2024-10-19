use std::io::{Read, Write};

struct Machine {
    code: String,
    ip: usize,
    memory: [i64; 30000],
    dp: usize,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
    buf: Vec<u8>,
}

impl Machine {
    fn new(code: String, input: Box<dyn Read>, output: Box<dyn Write>) -> Self {
        return Self {
            code,
            ip: 0,
            memory: [0; 30000],
            dp: 0,
            input,
            output,
            buf: Vec::with_capacity(1),
        };
    }

    fn execute(&mut self) {
        while self.ip < self.code.len() {
            if let Some(instruction) = self.code.chars().nth(self.ip) {
                match instruction {
                    '+' => self.memory[self.dp] += 1,
                    '-' => self.memory[self.dp] -= 1,
                    '>' => self.dp += 1,
                    '<' => self.dp -= 1,
                    ',' => self.read_char(),
                    '.' => self.put_char(),
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

                self.memory[self.dp] = self.buf[0] as i64;
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
