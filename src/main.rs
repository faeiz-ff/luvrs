pub mod lexer;
pub mod token;

use std::io::Write;

use lexer::Lexer;

fn main() {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("failed");

        let mut code = String::new();

        std::io::stdin().read_line(&mut code).expect("failed");

        let mut l = Lexer::new(code.trim());

        while let Some(tok) = l.lex() {
            println!("{:?}", tok);
        }
    }
}
