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
}
