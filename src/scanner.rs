use crate::lox::Lox;
use crate::token::{Token, TokenType};

pub struct Scanner {
    source: Vec<char>,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self, _lox: &mut Lox) -> Vec<Token> {
        // TODO: implement token scanning
        vec![Token {
            kind: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        }]
    }
}
