use bf_rust::Program;

#[test]
fn test_hello_world() {
    let program = String::from("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
    let mut machine = Program::new(program);
    assert_eq!(String::from("Hello World!\n"), machine.run().unwrap());
}