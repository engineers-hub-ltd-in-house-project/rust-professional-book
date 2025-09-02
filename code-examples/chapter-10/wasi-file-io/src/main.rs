use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    println!("Attempting to read file: {}", file_path);

    let content = fs::read_to_string(file_path)
        .expect("could not read file");

    println!("File content:\n{}", content);
}
