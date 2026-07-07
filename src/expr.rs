use crate::token::{LiteralValue, Token, TokenType};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token
    },
}

pub fn print(expr: &Expr) -> String {
    match &expr {
        Expr::Binary { left, operator, right } => {
            parenthesize(&operator.lexeme, &[left, right])
        }
        Expr::Grouping { expression } => parenthesize("group", &[expression]),
        Expr::Literal { value } => match value {
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
        },
        Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
        Expr::Variable { name } => name.lexeme.clone(),
    }
}

fn parenthesize(name: &str, exprs: &[&Expr]) -> String {
    let mut result = format!("({name}");
    for expr in exprs {
        result.push(' ');
        result.push_str(&print(expr));
    }
    result.push(')');
    result
}

/// Hand-built tree test: `(- 123) * (group 45.67)`
pub fn demo_expression() -> Expr {
    Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                kind: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123.0),
            }),
        }),
        operator: Token {
            kind: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        }),
    }
}
