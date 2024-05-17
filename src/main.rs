use std::env;
use std::fs;
use std::io;

use rlox::Environment;
use rlox::Literal;
use rlox::Scanner;
use rlox::Token;
use rlox::TokenType;
use rlox_expr::Binary;
use rlox_expr::Expression;
use rlox_expr::Printer;
use rlox_expr::Unary;
use rlox_expr::Visitor;

mod rlox;
mod rlox_expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let exit_code = match args.len() {
        1 => {
            match run_prompt() {
                Err(e) => { eprintln!("{:?}", e); 63 },
                Ok(code) => {
                    println!("RLox signing off!");
                    code
                },
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

    let expr = Expression::Binary(Binary {
        left: Box::new(Expression::Unary( Unary {
            operator: Token { token_type: TokenType::Minus, lexeme: "-".to_owned(), literal: Literal::None, line: 1 },
            right: Box::new(Expression::Literal(rlox_expr::Literal { value: rlox::Literal::Number(1234.0) }))
        })),
        right: Box::new(Expression::Grouping(
            rlox_expr::Grouping { expression: Box::new(Expression::Literal(rlox_expr::Literal { value: rlox::Literal::Number(45.67) })) }
        )),
        operator: Token { token_type: TokenType::Star, lexeme: "*".to_owned(), literal: Literal::None, line: 1 }
    });

    let printer = Printer{};
    println!("{}", printer.print(&expr));

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
    scanner.scan_tokens(environment);

    scanner.debug_print_tokens();
}

