use std::io::{self, BufRead, Write};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    fn reset_error(&mut self) {
        self.had_error = false;
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = crate::scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens(self);

        for token in tokens {
            println!("{token}");
        }
    }

    pub fn run_file(&mut self, path: &str) {
        let source = std::fs::read_to_string(path).expect("cannot read file");
        self.run(&source);

        if self.had_error {
            std::process::exit(65);
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
