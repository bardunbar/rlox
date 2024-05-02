use std::env;
use std::fs;
use std::io;

use rlox::Environment;
use rlox::Scanner;

mod rlox;

fn main() {
    let args: Vec<String> = env::args().collect();
    let exit_code = match args.len() {
        1 => {
            match run_prompt() {
                Err(e) => { eprintln!("{:?}", e); 63 },
                Ok(code) => code,
            }
        },
        2 => {
            match run_file(&args[1]) {
                Err(e) => { eprintln!("{:?}", e); 63 },
                Ok(code) => code,
            }
        },
        _ => {
            println!("Usage: rlox [script]");
            64
        }
    };

    std::process::exit(exit_code)
}

fn run_prompt() -> io::Result<i32> {

    let mut environment = Environment::new();
    let mut buffer = String::new();

    loop
    {
        io::stdin().read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        run(buffer.clone(), &mut environment);
        buffer.clear();
    }

    Ok(environment.get_exit_code())
}

fn run_file(path: &str) -> io::Result<i32>{
    let mut environment = Environment::new();
    let result = fs::read_to_string(path);
    run(result?, &mut environment);
    Ok(environment.get_exit_code())
}

fn run(source: String, environment: &mut Environment) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens(&environment);
}