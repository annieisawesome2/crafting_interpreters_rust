use std::rc::Rc;

use crate::callable::LoxCallable;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
}

#[derive(Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<dyn LoxCallable>),
}

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::String(s) => write!(f, "String({s:?})"),
            LiteralValue::Number(n) => write!(f, "Number({n:?})"),
            LiteralValue::Boolean(b) => write!(f, "Boolean({b:?})"),
            LiteralValue::Nil => write!(f, "Nil"),
            LiteralValue::Callable(_) => write!(f, "Callable"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.literal {
            None => "null".to_string(),
            Some(LiteralValue::String(s)) => s.clone(),
            Some(LiteralValue::Number(n)) => n.to_string(),
            Some(LiteralValue::Boolean(b)) => b.to_string(),
            Some(LiteralValue::Nil) => "nil".to_string(),
            Some(LiteralValue::Callable(_)) => "<fn>".to_string(),
        };
        write!(f, "{:?} {} {}", self.kind, self.lexeme, literal)
    }
}
