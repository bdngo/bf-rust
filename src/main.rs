use bf_rust::Program;
use bitvec::prelude::{self as bv, Lsb0};
use clap::{Args, Parser};
use std::{fs::File, io::Read, num::Wrapping, path::Path, process};

const MEM_SIZE: usize = 1 << 20;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Command {
    #[command(flatten)]
    group: Group,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Group {
    #[arg(short, long)]
    string: Option<String>,

    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Command::parse();
    let program = if let Some(input_str) = args.group.string {
        input_str
    } else if let Some(input_file) = args.group.file {
        let path = Path::new(&input_file);
        let display = path.display();

        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => s,
        }
    } else {
        panic!("invalid args");
    };
    let mut memory = [Wrapping(0u8); MEM_SIZE];
    let mut bitfield = bv::bitarr![0, MEM_SIZE];
    let mut machine = Program::new(program, &mut memory, &mut bitfield);
    match machine.run() {
        Ok(s) => print!("{}", s),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
