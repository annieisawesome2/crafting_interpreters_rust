use std::io::{self, BufRead, Write};

use crate::token::{Token, TokenType};

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
    interpreter: crate::interpreter::Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Lox {
            had_error: false,
            had_runtime_error: false,
            interpreter: crate::interpreter::Interpreter,
        }
    }

    pub fn error(&mut self, token: &Token, message: &str) {
        if token.kind == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    pub fn runtime_error(&mut self, token: &Token, message: &str) {
        if token.kind == TokenType::Eof {
            eprintln!("[line {}] Runtime error at end: {message}", token.line);
        } else {
            eprintln!(
                "[line {}] Runtime error at '{}': {message}",
                token.line, token.lexeme
            );
        }
        self.had_runtime_error = true;
    }

    pub fn error_at(&mut self, line: usize, message: &str) {
        eprintln!("[line {line}] Error: {message}");
        self.had_error = true;
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    fn reset_error(&mut self) {
        self.had_error = false;
        self.had_runtime_error = false;
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = crate::scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        let mut parser = crate::parser::Parser::new(tokens);
        let statements = parser.parse(self); 

        if self.had_error {
            return;
        }

        if let Some(stmts) = statements {
            if let Err(e) = self.interpreter.interpret(&stmts) {
                self.runtime_error(&e.token, &e.message); 
            }
        }
    }

    pub fn run_file(&mut self, path: &str) {
        let source = std::fs::read_to_string(path).expect("cannot read file");
        self.run(&source);

        if self.had_error {
            std::process::exit(65);
        }
        if self.had_runtime_error {
            std::process::exit(70);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut line = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    self.run(line.trim_end());
                    self.reset_error();
                }
                Err(e) => {
                    eprintln!("Error reading line: {e}");
                    break;
                }
            }
        }
    }
}
