use bitvec::prelude::{self as bv, Lsb0};
use std::{error::Error, fmt, io, num::Wrapping};

const MEM_SIZE: usize = 2 << 16;
const NUM_ITERATIONS: u32 = 1_000;

#[derive(Debug)]
struct BFError(String);

impl fmt::Display for BFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Interpreter error: {}", self.0)
    }
}

impl Error for BFError {}

pub struct Program {
    program: String,
    memory: [Wrapping<u8>; MEM_SIZE],
    bitfield: bv::BitArray,
}

impl Program {
    pub fn new(argv: &[String]) -> Result<Program, Box<dyn Error>> {
        if argv.len() != 2 {
            return Err(Box::new(BFError(String::from(
                "Incorrect number of arguments",
            ))));
        }
        let memory = [Wrapping(0u8); MEM_SIZE];
        let program = argv[1].clone();
        let bitfield = bv::bitarr![0, MEM_SIZE];
        Ok(Program {
            program,
            memory,
            bitfield,
        })
    }

    pub fn run(&mut self) -> Result<String, Box<dyn Error>> {
        let mut ptr = 0;
        let mut pc = 0;
        let mut num_iters: u32 = 0;
        let mut output_vec: Vec<u8> = Vec::new();

        while pc < self.program.len() && num_iters <= NUM_ITERATIONS {
            let prog_chars: Vec<char> = self.program.chars().collect();
            let c = prog_chars[pc];
            match c {
                '>' => {
                    let new_ptr = ptr + 1;
                    if !(0..MEM_SIZE).contains(&new_ptr)
                        && self.bitfield.get(ptr).as_deref() == Some(&true)
                    {
                        return Err(Box::new(BFError(String::from("Wrapping memory access"))));
                    }
                    ptr = new_ptr % MEM_SIZE;
                }
                '<' => {
                    let new_ptr = ptr - 1;
                    if !(0..MEM_SIZE).contains(&new_ptr)
                        && self.bitfield.get(ptr).as_deref() == Some(&true)
                    {
                        return Err(Box::new(BFError(String::from("Wrapping memory access"))));
                    }
                    ptr = new_ptr % MEM_SIZE;
                }
                '+' => {
                    *&mut self.memory[ptr] += Wrapping(1u8);
                    self.bitfield.set(ptr, true);
                }
                '-' => {
                    *&mut self.memory[ptr] -= Wrapping(1u8);
                    self.bitfield.set(ptr, true);
                }
                '.' => {
                    output_vec.push(self.memory[ptr].0);
                }
                ',' => {
                    *&mut self.memory[ptr] = Wrapping(self.process_input().unwrap());
                }
                '[' => {
                    if self.memory[ptr].0 == 0 {
                        match self.jump_fwd(pc) {
                            Ok(new_pc) => {
                                pc = new_pc;
                                num_iters += 1;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                ']' => {
                    if self.memory[ptr].0 != 0 {
                        match self.jump_bwd(pc) {
                            Ok(new_pc) => {
                                pc = new_pc;
                                num_iters += 1;
                            }
                            Err(e) => return Err(e),
                        }
                    }
                }
                _ => {}
            }
            pc += 1;
        }
        match String::from_utf8(output_vec) {
            Ok(s) => Ok(s),
            Err(_) => Err(Box::new(BFError(String::from("Bad UTF-8 encoding")))),
        }
    }

    fn process_input(&self) -> Result<u8, Box<dyn Error>> {
        let mut buf = String::new();
        let out;
        loop {
            println!("Input a character:");
            match io::stdin().read_line(&mut buf) {
                Ok(_) => {
                    if buf.len() != 2 {
                        eprintln!("Incorrect number of characters");
                    } else {
                        if let Some(c) = buf.bytes().next() {
                            out = c;
                            break;
                        } else {
                            eprintln!("Bad byte read");
                        }
                    }
                }
                Err(_) => return Err(Box::new(BFError(String::from("Bad IO read")))),
            }
            buf.clear();
        }
        Ok(out)
    }

    fn jump_fwd(&self, pc: usize) -> Result<usize, Box<dyn Error>> {
        let mut stack: Vec<char> = Vec::default();
        let mut new_pc = pc;
        let prog_chars: Vec<char> = self.program.chars().collect();
        let mut curr_char = prog_chars[new_pc];
        stack.push(curr_char);

        while new_pc < self.program.len() && !stack.is_empty() {
            new_pc += 1;
            curr_char = prog_chars[new_pc];
            if curr_char == '[' {
                stack.push(curr_char);
            } else if curr_char == ']' {
                stack.pop();
            }
        }
        if stack.is_empty() {
            Ok(new_pc)
        } else {
            Err(Box::new(BFError(String::from("Unbalanced program"))))
        }
    }

    fn jump_bwd(&self, pc: usize) -> Result<usize, Box<dyn Error>> {
        let mut stack: Vec<char> = Vec::default();
        let mut new_pc = pc;
        let prog_chars: Vec<char> = self.program.chars().collect();
        let mut curr_char = prog_chars[new_pc];
        stack.push(curr_char);

        while new_pc < self.program.len() && !stack.is_empty() {
            new_pc -= 1;
            curr_char = prog_chars[new_pc];
            if curr_char == ']' {
                stack.push(curr_char);
            } else if curr_char == '[' {
                stack.pop();
            }
        }
        if stack.is_empty() {
            Ok(new_pc)
        } else {
            Err(Box::new(BFError(String::from("Unbalanced program"))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_program() {
        let prog = Program::new(&[String::from(""), String::from("")]);
        let out = prog.unwrap().run();
        assert_eq!(String::default(), out.unwrap());
    }

    #[test]
    fn test_increment() {
        let prog = Program::new(&[String::from(""), String::from("+.")]);
        let out = prog.unwrap().run();
        let exp = &[0x01u8];
        assert_eq!(String::from_utf8_lossy(exp), out.unwrap());
    }

    #[test]
    #[ignore]
    fn test_decrement() {
        let prog = Program::new(&[String::from(""), String::from("-.")]);
        let out = prog.unwrap().run();
        let exp = &[255];
        assert_eq!(String::from_utf8_lossy(exp), out.unwrap());
    }

    #[test]
    fn test_infinite_loop() {
        let prog = Program::new(&[String::from(""), String::from("[]")]);
        let out = prog.unwrap().run();
        assert_eq!(String::default(), out.unwrap());
    }
}
