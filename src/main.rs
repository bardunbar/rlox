use std::env;
use std::fs;
use std::io;

use rlox::Scanner;

mod rlox;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => { run_prompt(); },
        2 => { run_file(&args[1]); },
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64)
        }
    }
}

fn run_prompt() -> io::Result<()> {

    let mut buffer = String::new();

    loop
    {
        io::stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        run(buffer.clone());
        buffer.clear();
    }

    Ok(())
}

fn run_file(path: &str) -> io::Result<()>{
    let result = fs::read_to_string(path);
    run(result?);
    Ok(())
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
}