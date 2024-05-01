use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}

fn run_prompt() {

}

fn run_file(path: &str) {
    let result = fs::read_to_string(path);

}

fn run(source: &str) {

}