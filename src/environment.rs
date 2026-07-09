use std::collections::HashMap;
use crate::token::LiteralValue;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    // global scope
    pub fn new() -> Self {
        Environment {
            enclosing: None, 
            values:HashMap::new(),
        }
    }

    // nested scope
    pub fn new_enclosing(enclosing: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(enclosing)), 
            values: HashMap::new(), 
        }
    }

    pub fn take_enclosing(mut self) -> Environment {
        *self.enclosing.take().expect("block scope must have enclosing")
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value); 
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn assign(&mut self, name:&str, value: LiteralValue) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value); 
            true
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)
        } else {
            false
        }
    }
}

