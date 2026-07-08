use std::collections::HashMap;
use crate::token::LiteralValue;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values:HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value); 
    }

    pub fn get(&self, name: &str) -> Option<LiteralValue> {
        self.values.get(name).cloned()
    }

    pub fn assign(&mut self, name:&str, value: LiteralValue) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value); 
            true
        } else {
            false
        }
    }
}

