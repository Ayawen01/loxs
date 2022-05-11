use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::{ast::LoxObject, error::LoxError, token::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LoxObject>,
    environment: Option<Rc<RefCell<Environment>>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment { values: HashMap::new(), environment: None }
    }

    pub fn from(environment: Rc<RefCell<Environment>>) -> Environment {
        Environment { values: HashMap::new(), environment: Some(environment) }
    }

    pub fn get(&self, name: Token) -> Result<LoxObject, LoxError> {
        let lexeme = name.clone().lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.get(&lexeme).unwrap().clone())
        } else {
            if let Some(environment) = &self.environment {
                environment.borrow().get(name)
            } else {
                Err(LoxError::RuntimeError { msg: format!("Undefined variable '{}'.", &lexeme).into(), line: name.line })
            }
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: LoxObject) -> Result<LoxObject, LoxError> {
        let lexeme = name.clone().lexeme.unwrap();
        if self.values.contains_key(&lexeme) {
            Ok(self.values.insert(lexeme, value).unwrap())
        } else {
            if let Some(environment) = &mut self.environment {
                environment.borrow_mut().assign(name, value)
            } else {
                Err(LoxError::RuntimeError { msg: format!("Undefined variable '{}'.", lexeme).into(), line: name.line })   
            }
        }
    }   
}