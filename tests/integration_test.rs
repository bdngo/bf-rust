use bf_rust::Program;

#[test]
fn test_hello_world() {
    let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let machine = Program::new(&[String::from(""), program]);
    assert_eq!(String::from("Hello World!\n"), machine.unwrap().run().unwrap());
}

#[test]
fn test_hello_world_2() {
    let program = String::from(">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]>-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.------.--------.>+.>+.");
    let machine = Program::new(&[String::from(""), program]);
    assert_eq!(String::from("Hello World!\n"), machine.unwrap().run().unwrap());
}
