mod lox;
mod scanner;
mod token;

use lox::Lox;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        lox.run_file(&args[1]);
    } else {
        lox.run_prompt();
    }
}
