use crate::expr::Expr;
use crate::token::{LiteralValue, Token, TokenType};

pub struct Interpreter; 

impl Interpreter {
    pub fn interpret(&mut self, expr:&Expr) -> Result<LiteralValue, String> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()), 
            Expr::Grouping { expression } => self.interpret(expression), 
            Expr::Unary { operator, right } => {
                let operand = self.interpret(right)?; 
                self.apply_unary(operator, operand)
            }

            Expr::Binary { left, operator, right } => {
                let left = self.interpret(left)?; 
                let right = self.interpret(right)?; 
                self.apply_binary(operator, left, right)
            }
        }
    }

    fn apply_unary(&mut self, operator: &Token, operand:LiteralValue) -> Result<LiteralValue, String> {
        match operator.kind {
            TokenType::Minus => match operand {
                LiteralValue::Number(n) =>  Ok(LiteralValue::Number(-n)), 
                _ => Err("Operand must be a number.".into()),
            }, 

            TokenType::Bang => Ok(LiteralValue::Boolean(!Self::is_truthy(&operand))), 
            _ => Err("Invalid unary operator.".into()),
        }
    }

    fn is_truthy(value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Nil => false, 
            LiteralValue::Boolean(b) => *b, 
            _ => true
        }
    }

    fn apply_binary(&self, operator: &Token, left: LiteralValue, right: LiteralValue) -> Result<LiteralValue, String> {
        match operator.kind {
            TokenType::Minus => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l-r))
            }, 

            TokenType::Slash => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l/r))
            }, 

            TokenType::Star => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l*r))
            }, 

            TokenType::Plus => {
                // can concatenate two strings
                match (&left, &right) {
                    (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                        Ok(LiteralValue::Number(l+r))
                    }

                    (LiteralValue::String(l), LiteralValue::String(r)) => {
                        Ok(LiteralValue::String(format!("{l}{r}")))
                    }
                    _ => Err("Operands must be two numbers or two strings.".into()),

                }
            }, 

            TokenType::Greater => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l>r))
            }, 

            TokenType::GreaterEqual => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l>=r))
            }, 

            TokenType::Less => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l<r))
            }, 

            TokenType::LessEqual => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l<=r))
            },

            TokenType::BangEqual => Ok(LiteralValue::Boolean(!Self::is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(LiteralValue::Boolean(Self::is_equal(&left, &right))),
            _ => Err("Invalid binary operator.".into())
        }
    }

    fn is_equal(a: &LiteralValue, b: &LiteralValue) -> bool {
        match (a, b) {
            (LiteralValue::Nil, LiteralValue::Nil) => true,
            (LiteralValue::Number(x), LiteralValue::Number(y)) => x == y,
            (LiteralValue::String(x), LiteralValue::String(y)) => x == y,
            (LiteralValue::Boolean(x), LiteralValue::Boolean(y)) => x == y,
            _ => false,
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<(), String> {
        match (left, right) {
            (LiteralValue::Number(_), LiteralValue::Number(_)) => Ok(()),
            _ => Err(format!("Operands must be numbers for '{}'.", operator.lexeme)),
        }
    }

    fn unwrap_numbers(left: &LiteralValue, right: &LiteralValue) -> (f64, f64) {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => (*l, *r),
            _ => unreachable!("check_number_operands should run first"),
        }
    }
    



}