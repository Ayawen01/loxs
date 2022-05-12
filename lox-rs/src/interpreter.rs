use std::{rc::Rc, cell::RefCell};

use crate::{
    ast::{VisitorExpr, Expr, LoxObject, LoxLiteral, VisitorStmt, Stmt},
    error::LoxError,
    token::{Token, TokenType}, environment::Environment,
};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>
}

impl Interpreter {
    #[inline]
    pub fn new() -> Interpreter {
        Interpreter { environment: Rc::new(RefCell::new(Environment::new())) }
    }

    #[inline]
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        
        Ok(())
    }
    
    #[inline]
    fn is_truthy(&self, value: &LoxObject) -> bool {
        match value {
            LoxObject::Nil => false,
            LoxObject::Bool(bool) => *bool,
            _ => true        
        }
    }

    #[inline]
    fn is_equal(&self, l: &LoxObject, r: &LoxObject) -> bool {
        match (l, r) {
            (LoxObject::Nil, LoxObject::Nil) => true,
            (LoxObject::Bool(l), LoxObject::Bool(r)) => l == r,
            (LoxObject::String(l), LoxObject::String(r)) => l == r,
            (LoxObject::Number(l), LoxObject::Number(r)) => l == r,
            _ => false
        }
    }

    #[inline]
    fn stringify(&self, value: LoxObject) -> String {
        match value {
            LoxObject::String(str) => str,
            LoxObject::Number(num) => num.to_string(),
            LoxObject::Bool(bool) => bool.to_string(),
            LoxObject::Nil => "nil".to_owned()
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Rc<RefCell<Environment>>) -> Result<(), LoxError> {
        let previous = environment.clone();

        self.environment = Rc::new(RefCell::new(Environment::from(environment)));

        for stmt in statements {
            self.execute(stmt)?;
        }

        self.environment = previous;

        Ok(())
    }
}

impl VisitorExpr<LoxObject> for Interpreter {
    fn visit_assign_expr(&mut self, name: Token, value: Box<Expr>) -> Result<LoxObject, LoxError> {
        let value = self.evaluate(*value)?;
        self.environment.borrow_mut().assign(name, value)
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxObject, LoxError> {
        let left = self.evaluate(*left)?;
        let right = self.evaluate(*right)?;
        
        match operator.r#type {
            TokenType::BangEqual => Ok(LoxObject::Bool(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(LoxObject::Bool(self.is_equal(&left, &right))),
            TokenType::Greater => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Bool(l > r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::GreaterEqual => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Bool(l >= r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::Less => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Bool(l < r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::LessEqual => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Bool(l <= r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            },
            TokenType::Minus => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Number(l - r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            TokenType::Plus => {
                match (left.clone(), right.clone()) {
                    (LoxObject::Number(l), LoxObject::Number(r)) => Ok(LoxObject::Number(l + r)),
                    (LoxObject::String(l), LoxObject::String(r)) => Ok(LoxObject::String(l + &r)),
                    _ => {
                        let msg = format!("{} and {} must both be numbers or both be strings.", left, right);
                        Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                    }
                }
            }
            TokenType::Slash => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Number(l / r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            TokenType::Star => {
                if let (LoxObject::Number(l), LoxObject::Number(r)) = (left.clone(), right.clone()) {
                    Ok(LoxObject::Number(l * r))
                } else {
                    let msg = format!("{} and {} must be numbers.", left, right);
                    Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line })
                }
            }
            _ => {
                Ok(LoxObject::Nil)
            }
        }
    }

    fn visit_call_expr(&self, callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Result<LoxObject, LoxError> {
        todo!()
    }

    fn visit_get_expr(&self, object: Box<Expr>, name: Token) -> Result<LoxObject, LoxError> {
        todo!()
    }

    #[inline]
    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> Result<LoxObject, LoxError> {
        self.evaluate(*expression)
    }

    #[inline]
    fn visit_literal_expr(&self, value: LoxLiteral) -> Result<LoxObject, LoxError> {
        Ok(match value {
            LoxLiteral::String(str) => LoxObject::String(str),
            LoxLiteral::Number(num) => LoxObject::Number(num),
            LoxLiteral::Bool(bool) => LoxObject::Bool(bool),
            LoxLiteral::Nil => LoxObject::Nil
        })
    }

    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxObject, LoxError> {
        let left = self.evaluate(*left)?;

        if operator.r#type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(*right)
    }

    fn visit_set_expr(&self, object: Box<Expr>, name: Token, value: Box<Expr>) -> Result<LoxObject, LoxError> {
        todo!()
    }

    fn visit_super_expr(&self, keyword: Token, method: Token) -> Result<LoxObject, LoxError> {
        todo!()
    }

    fn visit_this_expr(&self, keyword: Token) -> Result<LoxObject, LoxError> {
        todo!()
    }

    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Result<LoxObject, LoxError> {
        let right = self.evaluate(*right)?;
        
        Ok(match operator.r#type {
            TokenType::Bang => LoxObject::Bool(!self.is_truthy(&right)),
            TokenType::Minus => {
                if let LoxObject::Number(num) = right {
                    LoxObject::Number(-num)
                } else {
                    let msg = format!("{} must be a number.", right);
                    return Err(LoxError::RuntimeError { msg: msg.into(), line: operator.line });
                }
            }
            _ => LoxObject::Nil
        })
    }

    #[inline]
    fn visit_variable_expr(&mut self, name: Token) -> Result<LoxObject, LoxError> {
        self.environment.borrow().get(name)
    }
}

impl VisitorStmt<()> for Interpreter {
    #[inline]
    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) -> Result<(), LoxError> {
        self.execute_block(statements, self.environment.clone())
    }

    fn visit_class_stmt(&self, name: Token, superclass: Option<Expr>, methods: Vec<Stmt>) -> Result<(), LoxError> {
        todo!()
    }

    #[inline]
    fn visit_expression_stmt(&mut self, expression: Expr) -> Result<(), LoxError> {
        match self.evaluate(expression) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_if_stmt(&mut self, condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Result<(), LoxError> {
        let v = self.evaluate(condition)?;
        if self.is_truthy(&v) {
            self.execute(*then_branch)?;
        } else if let Some(else_branch) = else_branch {
            self.execute(*else_branch)?;
        }
        Ok(())
    }

    #[inline]
    fn visit_print_stmt(&mut self, expression: Expr) -> Result<(), LoxError> {
        match self.evaluate(expression) {
            Ok(expr) => {
                println!("{}", self.stringify(expr));
                Ok(())
            }
            Err(e) => Err(e)
        }
    }

    fn visit_return_stmt(&self, keyword: Token, value: Option<Expr>) -> Result<(), LoxError> {
        todo!()
    }

    fn visit_var_stmt(&mut self, name: Token, initializer: Option<Expr>) -> Result<(), LoxError> {
        if let Some(expr) = initializer {
            match self.evaluate(expr) {
                Ok(v) => {
                    self.environment.borrow_mut().define(name.lexeme.unwrap(), v);
                }
                Err(e) => return Err(e)
            };
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: Expr, body: Box<Stmt>) -> Result<(), LoxError> {
        loop {
            let bool = self.evaluate(condition.clone())?;

            if self.is_truthy(&bool) {
                self.execute(*body.clone())?;
            } else {
                break
            }
        }

        Ok(())
    }
}