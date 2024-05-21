use bf_rust::Program;
use bitvec::prelude::{self as bv, Lsb0};
use std::{env, num::Wrapping, process};

const MEM_SIZE: usize = 1 << 20;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut memory = [Wrapping(0u8); MEM_SIZE];
    let mut bitfield = bv::bitarr![0, MEM_SIZE];
    let machine = Program::new(&argv, &mut memory, &mut bitfield);
    match machine {
        Ok(mut p) => match p.run() {
            Ok(s) => print!("{}", s),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
