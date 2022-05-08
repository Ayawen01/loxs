use std::collections::HashMap;

use crate::{
    ast::{VisitorExpr, Expr, LoxValue, LoxLiteral, VisitorStmt, Stmt},
    error::LoxError,
    token::{Token, TokenType}, environment::Environment,
};

pub struct Interpreter {
    values: HashMap<String, LoxValue>,
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { values: HashMap::new(), environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        
        Ok(())
    }
    
    #[inline]
    fn is_truthy(&self, value: &LoxValue) -> bool {
        match value {
            LoxValue::Nil => false,
            LoxValue::Bool(bool) => *bool,
            _ => true        
        }
    }

    #[inline]
    fn is_equal(&self, l: &LoxValue, r: &LoxValue) -> bool {
        match (l, r) {
            (LoxValue::Nil, LoxValue::Nil) => true,
            (LoxValue::Bool(l), LoxValue::Bool(r)) => l == r,
            (LoxValue::String(l), LoxValue::String(r)) => l == r,
            (LoxValue::Number(l), LoxValue::Number(r)) => l == r,
            _ => false
        }
    }

    #[inline]
    fn stringify(&self, value: LoxValue) -> String {
        match value {
            LoxValue::String(str) => str,
            LoxValue::Number(num) => num.to_string(),
            LoxValue::Bool(bool) => bool.to_string(),
            LoxValue::Nil => "nil".to_owned()
        }
    }
}

impl VisitorExpr<LoxValue> for Interpreter {
    fn visit_assign_expr(&self, name: Token, value: Box<Expr>) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxValue, LoxError> {
        let left = match self.evaluate(*left) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        let right = match self.evaluate(*right) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        
        match operator.r#type {
            TokenType::BangEqual => Ok(LoxValue::Bool(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(LoxValue::Bool(self.is_equal(&left, &right))),
            TokenType::Greater => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Bool(l > r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::GreaterEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Bool(l >= r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::Less => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Bool(l < r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::LessEqual => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Bool(l <= r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::Minus => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Number(l - r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            TokenType::Plus => {
                match (left.clone(), right.clone()) {
                    (LoxValue::Number(l), LoxValue::Number(r)) => Ok(LoxValue::Number(l + r)),
                    (LoxValue::String(l), LoxValue::String(r)) => Ok(LoxValue::String(l + &r)),
                    _ => {
                        let msg = format!("{} and {} must both be numbers or both be strings.", left, right);
                        Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                    }
                }
            }
            TokenType::Slash => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Number(l / r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            TokenType::Star => {
                if let (LoxValue::Number(l), LoxValue::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxValue::Number(l * r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            _ => {
                Ok(LoxValue::Nil)
            }
        }
    }

    fn visit_call_expr(&self, callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_get_expr(&self, object: Box<Expr>, name: Token) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> Result<LoxValue, LoxError> {
        self.evaluate(*expression)
    }

    fn visit_literal_expr(&self, value: LoxLiteral) -> Result<LoxValue, LoxError> {
        Ok(match value {
            LoxLiteral::String(str) => LoxValue::String(str),
            LoxLiteral::Number(num) => LoxValue::Number(num),
            LoxLiteral::Bool(bool) => LoxValue::Bool(bool),
            LoxLiteral::Nil => LoxValue::Nil
        })
    }

    fn visit_logical_expr(&self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_set_expr(&self, object: Box<Expr>, name: Token, value: Box<Expr>) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_super_expr(&self, keyword: Token, method: Token) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_this_expr(&self, keyword: Token) -> Result<LoxValue, LoxError> {
        todo!()
    }

    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Result<LoxValue, LoxError> {
        let right = match self.evaluate(*right) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        
        Ok(match operator.r#type {
            TokenType::Bang => LoxValue::Bool(!self.is_truthy(&right)),
            TokenType::Minus => {
                if let LoxValue::Number(num) = right {
                    LoxValue::Number(-num)
                } else {
                    let msg = format!("{} must be a number.", right);
                    return Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line });
                }
            }
            _ => LoxValue::Nil
        })
    }

    fn visit_variable_expr(&mut self, name: Token) -> Result<LoxValue, LoxError> {
        self.environment.get(name)
    }
}

impl VisitorStmt<()> for Interpreter {
    fn visit_block_stmt(&self, statements: Vec<crate::ast::Stmt>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_class_stmt(&self, name: Token, superclass: Option<Expr>, methods: Vec<crate::ast::Stmt>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_expression_stmt(&mut self, expression: Expr) -> Result<(), LoxError> {
        match self.evaluate(expression) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<crate::ast::Stmt>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_if_stmt(&self, condition: Expr, then_branch: Box<crate::ast::Stmt>, else_branch: Option<Box<crate::ast::Stmt>>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_print_stmt(&mut self, expression: Expr) -> Result<(), LoxError> {
        match self.evaluate(expression) {
            Ok(expr) => println!("{}", self.stringify(expr)),
            Err(e) => return Err(e)
        };
        Ok(())
    }

    fn visit_return_stmt(&self, keyword: Token, value: Option<Expr>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> Result<(), LoxError> {
        if let Some(expr) = initializer {
            match self.evaluate(expr) {
                Ok(v) => {
                    self.environment.define(name.lexeme.unwrap(), v);
                }
                Err(e) => return Err(e)
            };
        }
        Ok(())
    }

    fn visit_while_stmt(&self, condition: Expr, body: Box<crate::ast::Stmt>) -> Result<(), LoxError> {
        todo!()
    }
}