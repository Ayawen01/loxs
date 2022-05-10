use std::collections::HashMap;

use crate::{ast::LoxObject, error::LoxError, token::Token};

pub struct Environment {
    values: HashMap<String, LoxObject>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new() }
    }

    pub fn get(&self, name: Token) -> Result<LoxObject, LoxError> {
        let lexeme = name.lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.get(&lexeme).unwrap().clone())
        } else {
            Err(LoxError::RuntimeError { msg: format!("Undefined variable '{}'.", &lexeme).into(), line: name.line })
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: LoxObject) -> Result<LoxObject, LoxError> {
        let lexeme = name.lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.insert(lexeme, value).unwrap())
        } else {
            Err(LoxError::RuntimeError { msg: format!("Undefined variable '{}'.", lexeme).into(), line: name.line })   
        }
    }   
}