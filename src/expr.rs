use crate::token::{LiteralValue, Token, TokenType};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: Box<Expr>, 
        operator: Token, 
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token
    },
    Assign {
        name: Token, 
        value: Box<Expr>, 
    }
}

pub fn print(expr: &Expr) -> String {
    match &expr {
        Expr::Binary { left, operator, right } => {
            parenthesize(&operator.lexeme, &[left, right])
        }
        Expr::Call { callee, arguments, .. } => {
            let mut parts: Vec<&Expr> = vec![callee.as_ref()];
            for arg in arguments {
                parts.push(arg);
            }
            parenthesize("call", &parts)
        }
        Expr::Grouping { expression } => parenthesize("group", &[expression]),
        Expr::Literal { value } => match value {
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable(_) => "<fn>".to_string(),
        },
        Expr::Unary { operator, right } => parenthesize(&operator.lexeme, &[right]),
        Expr::Variable { name } => name.lexeme.clone(),
        Expr::Assign { name, value } => {format!("(= {} {})", name.lexeme, print(value))}
        Expr::Logical { left, operator, right } => {
            parenthesize(&operator.lexeme, &[left, right])
        }
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
