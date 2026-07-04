use crate::lox::Lox;
use crate::token::{LiteralValue, Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }

        self.tokens.push(Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        std::mem::take(&mut self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let kind = if self.matches_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(kind, None);
            }
            '=' => {
                let kind = if self.matches_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(kind, None);
            }
            '<' => {
                let kind = if self.matches_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(kind, None);
            }
            '>' => {
                let kind = if self.matches_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(kind, None);
            }
            '/' => {
                if self.matches_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(lox),
            c if Self::is_digit(c) => self.number(),
            _ => lox.error(self.line, "Unexpected character."),
        }
    }

    fn advance(&mut self) -> char {
        if !self.is_at_end() {
            let c = self.source[self.current];
            self.current += 1;
            c
        } else {
            '\0' // call when not at end
        }
    }

    fn add_token(&mut self, kind: TokenType, literal: Option<LiteralValue>) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            kind,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    fn matches_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false; 
        }

        if self.source[self.current] != expected {
            return false; 
        }

        self.current += 1; 
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1; 
            }
            self.advance(); 
        }

        if self.is_at_end() {
            lox.error(self.line, "unterminated string.");
            return; 
        }

        self.advance(); // go past closing quote

        let value: String = self.source[self.start + 1..self.current - 1].iter().collect(); 

        self.add_token(
            TokenType::String, 
            Some(LiteralValue::String(value))
        ); 
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance(); 
        }

        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance(); 

            while Self::is_digit(self.peek()) {
                self.advance(); 
            }
        }

        let lexeme:String = self.source[self.start..self.current].iter().collect(); 
        let value: f64 = lexeme.parse().expect("invalid number"); 

        self.add_token(TokenType::Number, Some(LiteralValue::Number(value)));
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current+1]
        }
    }
}
