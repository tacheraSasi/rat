mod print;

// use print;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    if let Err(e) = print::print_file(filename) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
