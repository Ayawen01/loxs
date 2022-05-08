use std::collections::HashMap;

use crate::{ast::LoxValue, error::LoxError, token::Token};

pub struct Environment {
    values: HashMap<String, LoxValue>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn get(&self, name: Token) -> Result<LoxValue, LoxError> {
        let lexeme = name.lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.get(&lexeme).unwrap().clone())
        } else {
            Err(LoxError::RuntimeError { msg: format!("Undefined variable '{}'.", &lexeme).into(), line: name.line })
        }
    }

    pub fn define(&mut self, name: String, value: LoxValue) {
        self.values.insert(name, value);
    }
}