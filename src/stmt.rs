use crate::expr::Expr; 
use crate::token::Token;

pub enum Stmt {
    Expression { expression: Box<Expr> }, 
    Print { expression: Box<Expr> },
    Var {
        name: Token, 
        initializer: Option<Box<Expr>>,
    },
}