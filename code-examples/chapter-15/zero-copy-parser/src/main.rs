use std::fs;
use zero_copy_parser::parse_env;

fn main() {
    let file_path = "example.env";
    println!("Parsing file: {}", file_path);

    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };

    match parse_env(&content) {
        Ok((remaining, map)) => {
            println!("Successfully parsed .env file:");
            for (key, value) in map {
                println!("  {} = \"{}\"", key, value);
            }
            if !remaining.trim().is_empty() {
                println!("\nWarning: some input was not parsed:");
                println!("{}", remaining);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse .env file: {:?}", e);
        }
    }
}