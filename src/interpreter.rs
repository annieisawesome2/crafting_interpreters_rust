use crate::expr::Expr;
use crate::token::{LiteralValue, Token, TokenType};
use crate::stmt::Stmt;
use crate::environment::Environment; 

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?; 
        }
        Ok(())
    }
    
    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression { expression } => {
                self.evaluate(expression)?;
                Ok(())
            }

            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?; 
                println!("{}", Self::stringify(&value)); 
                Ok(())
            }

            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(init) => self.evaluate(init)?,
                    None => LiteralValue::Nil,
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(())
            }

            Stmt::Block { statements } => {
                self.execute_block(statements)
            }

            Stmt::If {
                condition, 
                then_branch, 
                else_branch,
            } => {
                if Self::is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)
                } else {
                    Ok(())
                }
            }

            Stmt::While { condition, body } => {
                while Self::is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
                Ok(())
            }

        }
    }

    
    fn execute_block(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        let enclosing = std::mem::replace(&mut self.environment, Environment::new());
        self.environment = Environment::new_enclosing(enclosing);
        let result = self.execute_all(statements);
        let block_env = std::mem::replace(&mut self.environment, Environment::new());
        self.environment = block_env.take_enclosing();
        result
    }

    fn execute_all(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<LiteralValue, RuntimeError> {
        match expr {
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let operand = self.evaluate(right)?;
                self.apply_unary(operator, operand)
            }

            Expr::Binary { left, operator, right } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                self.apply_binary(operator, left, right)
            }

            Expr::Variable { name } => {
                if let Some(value) = self.environment.get(&name.lexeme) {
                    Ok(value)
                } else {
                    Err(Self::runtime_error(
                        name,
                        format!("Undefined variable '{}'.", name.lexeme),
                    ))
                }
            }

            Expr::Assign { name, value} => {
                let value = self.evaluate(value)?;
                if self.environment.assign(&name.lexeme, value.clone()) {
                    Ok(value) // value is assigned
                } else {
                    Err(Self::runtime_error(
                        name,
                        format!("Undefined variable '{}'.", name.lexeme),
                    ))
                }
            }

            Expr::Logical { left, operator, right } => {
                let left_val = self.evaluate(left)?;

                if operator.kind == TokenType::Or {
                    if Self::is_truthy(&left_val) {
                        return Ok(left_val);
                    }
                } else if !Self::is_truthy(&left_val) {
                    return Ok(left_val);
                }

                self.evaluate(right)
            }

            Expr::Call { callee, paren, arguments } => {
                let callee = self.evaluate(callee)?;

                let mut args = Vec::new();
                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                let LiteralValue::Callable(function) = callee else {
                    return Err(Self::runtime_error(
                        paren,
                        "Can only call functions and classes.",
                    ));
                };

                if args.len() != function.arity() {
                    return Err(Self::runtime_error(
                        paren,
                        format!(
                            "Expected {} arguments but got {}.",
                            function.arity(),
                            args.len()
                        ),
                    ));
                }

                function.call(self, args)
            }
        }
    }

    fn apply_unary(
        &mut self,
        operator: &Token,
        operand: LiteralValue,
    ) -> Result<LiteralValue, RuntimeError> {
        match operator.kind {
            TokenType::Minus => {
                Self::check_number_operand(operator, &operand)?;
                match operand {
                    LiteralValue::Number(n) => Ok(LiteralValue::Number(-n)),
                    _ => unreachable!("check_number_operand should run first"),
                }
            }

            TokenType::Bang => Ok(LiteralValue::Boolean(!Self::is_truthy(&operand))),
            _ => Err(Self::runtime_error(operator, "Invalid unary operator.")),
        }
    }

    fn is_truthy(value: &LiteralValue) -> bool {
        match value {
            LiteralValue::Nil => false,
            LiteralValue::Boolean(b) => *b,
            _ => true,
        }
    }

    fn apply_binary(
        &self,
        operator: &Token,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, RuntimeError> {
        match operator.kind {
            TokenType::Minus => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l - r))
            }

            TokenType::Slash => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l / r))
            }

            TokenType::Star => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Number(l * r))
            }

            TokenType::Plus => match (&left, &right) {
                (LiteralValue::Number(l), LiteralValue::Number(r)) => {
                    Ok(LiteralValue::Number(l + r))
                }

                (LiteralValue::String(l), LiteralValue::String(r)) => {
                    Ok(LiteralValue::String(format!("{l}{r}")))
                }
                _ => Err(Self::runtime_error(
                    operator,
                    "Operands must be two numbers or two strings.",
                )),
            },

            TokenType::Greater => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l > r))
            }

            TokenType::GreaterEqual => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l >= r))
            }

            TokenType::Less => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l < r))
            }

            TokenType::LessEqual => {
                Self::check_number_operands(operator, &left, &right)?;
                let (l, r) = Self::unwrap_numbers(&left, &right);
                Ok(LiteralValue::Boolean(l <= r))
            }

            TokenType::BangEqual => Ok(LiteralValue::Boolean(!Self::is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(LiteralValue::Boolean(Self::is_equal(&left, &right))),
            _ => Err(Self::runtime_error(operator, "Invalid binary operator.")),
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

    fn check_number_operand(operator: &Token, operand: &LiteralValue) -> Result<(), RuntimeError> {
        if matches!(operand, LiteralValue::Number(_)) {
            Ok(())
        } else {
            Err(Self::runtime_error(operator, "Operand must be a number."))
        }
    }

    fn check_number_operands(
        operator: &Token,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<(), RuntimeError> {
        if matches!(left, LiteralValue::Number(_)) && matches!(right, LiteralValue::Number(_)) {
            Ok(())
        } else {
            Err(Self::runtime_error(operator, "Operands must be numbers."))
        }
    }

    fn unwrap_numbers(left: &LiteralValue, right: &LiteralValue) -> (f64, f64) {
        match (left, right) {
            (LiteralValue::Number(l), LiteralValue::Number(r)) => (*l, *r),
            _ => unreachable!("check_number_operands should run first"),
        }
    }

    fn stringify(value: &LiteralValue) -> String {
        match value {
            LiteralValue::Nil => "nil".into(),
            LiteralValue::Boolean(b) => b.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Number(n) => {
                let text = n.to_string();
                if text.ends_with(".0") {
                    text[..text.len() - 2].to_string()
                } else {
                    text
                }
            }
            LiteralValue::Callable(_) => "<fn>".into(),
        }
    }

    fn runtime_error(token: &Token, message: impl Into<String>) -> RuntimeError {
        RuntimeError {
            token: token.clone(),
            message: message.into(),
        }
    }
}
