use crate::{
    ast::{VisitorExpr, Expr, LoxObject, LoxLiteral, VisitorStmt, Stmt},
    error::LoxError,
    token::{Token, TokenType}, environment::Environment,
};

pub struct Interpreter {
    environment: Environment
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { environment: Environment::new() }
    }

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
}

impl VisitorExpr<LoxObject> for Interpreter {
    fn visit_assign_expr(&mut self, name: Token, value: Box<Expr>) -> Result<LoxObject, LoxError> {
        let value = match self.evaluate(*value) {
            Ok(value) => value,
            Err(e) => return Err(e)
        };
        
        if let Err(e) = self.environment.assign(name, value) {
            Err(e)
        } else {
            Ok(LoxObject::Nil)
        }
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxObject, LoxError> {
        let left = match self.evaluate(*left) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        let right = match self.evaluate(*right) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        
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

    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> Result<LoxObject, LoxError> {
        self.evaluate(*expression)
    }

    fn visit_literal_expr(&self, value: LoxLiteral) -> Result<LoxObject, LoxError> {
        Ok(match value {
            LoxLiteral::String(str) => LoxObject::String(str),
            LoxLiteral::Number(num) => LoxObject::Number(num),
            LoxLiteral::Bool(bool) => LoxObject::Bool(bool),
            LoxLiteral::Nil => LoxObject::Nil
        })
    }

    fn visit_logical_expr(&self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<LoxObject, LoxError> {
        todo!()
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
        let right = match self.evaluate(*right) {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };
        
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

    fn visit_variable_expr(&mut self, name: Token) -> Result<LoxObject, LoxError> {
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