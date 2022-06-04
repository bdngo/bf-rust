use std::num::Wrapping;
use std::io::{self, Error, ErrorKind};

const MEM_SIZE: usize = 32_768;
const NUM_ITERATIONS: u32 = 1_000;

pub struct Program {
    program: String,
    memory: [Wrapping<u8>; MEM_SIZE],
}

impl Program {
    pub fn new(program: String) -> Program {
        let memory = [Wrapping(0u8); MEM_SIZE];
        Program { program, memory }
    }

    pub fn run(&mut self) -> Result<String, io::Error> {
        let mut ptr = 0; 
        let mut pc = 0;
        let mut num_iters: u32 = 0;
        let mut output_vec: Vec<u8> = Vec::new();
        if !self.is_matched() {
            return Err(Error::new(ErrorKind::Other, "Unmatched brackets"));
        }

        while pc < self.program.len() && num_iters <= NUM_ITERATIONS {
            let prog_chars: Vec<char> = self.program.chars().collect();
            let c = prog_chars[pc];
            match c {
                '>' => {
                    ptr += 1;
                },
                '<' => {
                    ptr -= 1;
                },
                '+' => {
                    *&mut self.memory[ptr] += Wrapping(1u8);
                },
                '-' => {
                    *&mut self.memory[ptr] -= Wrapping(1u8);
                },
                '.' => {
                    output_vec.push(self.memory[ptr].0);
                },
                ',' => {
                    *&mut self.memory[ptr] = Wrapping(self.process_input().unwrap());
                },
                '[' => {
                    if self.memory[ptr].0 == 0 {
                        while prog_chars[pc] != ']' {
                            pc += 1;
                        }
                        num_iters += 1;
                        continue;
                    }
                },
                ']' => {
                    if self.memory[ptr].0 != 0 {
                        while prog_chars[pc] != '[' {
                            pc -= 1;
                        }
                        num_iters += 1;
                        continue;
                    }
                }
                _ => continue,
            }
            pc += 1;
        }
        match String::from_utf8(output_vec) {
            Ok(s) => Ok(s),
            Err(e) => Err(Error::new(ErrorKind::Other, e)),
        }
    }

    fn process_input(&self) -> Result<u8, io::Error> {
        let mut buf = String::new();
        let out;
        loop {
            println!("Input a character:");
            match io::stdin().read_line(&mut buf) {
                Ok(_) => {
                    if buf.len() != 1 {
                        println!("Incorrect number of characters");
                    } else {
                        if let Some(c) = buf.bytes().next() {
                            out = c;
                            break;
                        } else {
                            println!("Bad byte read");
                        }
                    }
                },
                Err(e) => {
                    return Err(e)
                },
            }
        }
        Ok(out)
    }

    fn is_matched(&self) -> bool {
        let mut stack = Vec::default();
        for c in self.program.chars() {
            if c == '[' {
                stack.push(c);
            } else if c == ']' {
                stack.pop();
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_program() {
        let mut prog = Program::new(String::default());
        let out = prog.run();
        assert_eq!(String::default(), out.unwrap());
    }

    #[test]
    fn test_increment() {
        let mut prog = Program::new(String::from("+."));
        let out = prog.run();
        let exp = &[0x01u8];
        assert_eq!(String::from_utf8_lossy(exp), out.unwrap());
    }

    #[test]
    #[ignore]
    fn test_decrement() {
        let mut prog = Program::new(String::from("-."));
        let out = prog.run();
        let exp = &[255];
        assert_eq!(String::from_utf8_lossy(exp), out.unwrap());
    }

    #[test]
    fn test_infinite_loop() {
        let mut prog = Program::new(String::from("[]"));
        let out = prog.run();
        assert_eq!(String::default(), out.unwrap());
    }
}
