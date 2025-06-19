
extern crate scanner;

use scanner::Scanner;
use scanner::TokenType;

fn main() {
    println!("Code:");
    let mut source = String::from("");
    std::io::stdin().read_line(&mut source).expect("Failed to read the line!");

    let mut scanner = Scanner::new(source);
    loop {
        let token = scanner.scan_token();
        println!("{:?}", token);
        if token.token_type == TokenType::Eof {
            break;
        }
    }
}

