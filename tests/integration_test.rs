use bf_rust::Program;
use bitvec::prelude::{self as bv, Lsb0};
use std::num::Wrapping;

const MEM_SIZE: usize = 1 << 20;

#[test]
fn test_hello_world() {
    let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let mut memory = [Wrapping(0u8); MEM_SIZE];
    let mut bitfield = bv::bitarr![0, MEM_SIZE];
    let machine = Program::new(&[String::from(""), program], &mut memory, &mut bitfield);
    assert_eq!(
        String::from("Hello World!\n"),
        machine.unwrap().run().unwrap()
    );
}

#[test]
fn test_hello_world_2() {
    let program = String::from(">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.");
    let mut memory = [Wrapping(0u8); MEM_SIZE];
    let mut bitfield = bv::bitarr![0, MEM_SIZE];
    let machine = Program::new(&[String::from(""), program], &mut memory, &mut bitfield);
    assert_eq!(
        String::from("Hello World!\n"),
        machine.unwrap().run().unwrap()
    );
}
