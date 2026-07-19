use crate::interpreter::{Interpreter, RuntimeError};
use crate::token::LiteralValue;

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LiteralValue>,
    ) -> Result<LiteralValue, RuntimeError>;
}
