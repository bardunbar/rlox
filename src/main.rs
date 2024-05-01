use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("Run Prompt"),
        2 => println!("Run Script: {}", args[1]),
        _ => println!("Usage: rlox [script]")
    }
}
