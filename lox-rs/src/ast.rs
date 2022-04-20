use std::fmt::{Debug, Display};

use crate::{token::Token, error::LoxError};

pub trait VisitorExpr<R> {
    fn visit_assign_expr(&self, name: Token, value: Box<Expr>) -> Result<R, LoxError>;
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<R, LoxError>;
    fn visit_call_expr(&self, callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Result<R, LoxError>;
    fn visit_get_expr(&self, object: Box<Expr>, name: Token) -> Result<R, LoxError>;
    fn visit_grouping_expr(&mut self, expression: Box<Expr>) -> Result<R, LoxError>;
    fn visit_literal_expr(&self, value: LoxLiteral) -> Result<R, LoxError>;
    fn visit_logical_expr(&self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<R, LoxError>;
    fn visit_set_expr(&self, object: Box<Expr>, name: Token, value: Box<Expr>) -> Result<R, LoxError>;
    fn visit_super_expr(&self, keyword: Token, method: Token) -> Result<R, LoxError>;
    fn visit_this_expr(&self, keyword: Token) -> Result<R, LoxError>;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Result<R, LoxError>;
    fn visit_variable_expr(&self, name: Token) -> Result<R, LoxError>;

    fn evaluate(&mut self, expr: Expr) -> Result<R, LoxError> {
        match expr {
            Expr::Assign { name, value } => self.visit_assign_expr(name, value),
            Expr::Binary { left, operator, right } => self.visit_binary_expr(left, operator, right),
            Expr::Call { callee, paren, arguments } => self.visit_call_expr(callee, paren, arguments),
            Expr::Get { object, name } => self.visit_get_expr(object, name),
            Expr::Grouping { expression } => self.visit_grouping_expr(expression),
            Expr::Literal { value } => self.visit_literal_expr(value),
            Expr::Logical { left, operator, right } => self.visit_logical_expr(left, operator, right),
            Expr::Set { object, name, value } => self.visit_set_expr(object, name, value),
            Expr::Super { keyword, method } => self.visit_super_expr(keyword, method),
            Expr::This { keyword } => self.visit_this_expr(keyword),
            Expr::Unary { operator, right } => self.visit_unary_expr(operator, right),
            Expr::Variable { name } => self.visit_variable_expr(name),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>
    },
    Get {
        object: Box<Expr>,
        name: Token
    },
    Grouping {
        expression: Box<Expr>
    },
    Literal {
        value: LoxLiteral
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>
    },
    Super {
        keyword: Token,
        method: Token
    },
    This {
        keyword: Token
    },
    Unary {
        operator: Token,
        right: Box<Expr>
    },
    Variable {
        name: Token
    }
}

#[derive(Debug)]
pub enum LoxLiteral {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

#[derive(Clone)]
pub enum LoxValue {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl Display for LoxValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = match self {
            LoxValue::String(_) => "string",
            LoxValue::Number(_) => "number",
            LoxValue::Bool(_) => "bool",
            LoxValue::Nil => "nil",
        };
        write!(f, "{}", r)
    }
}

pub trait VisitorStmt<R> {
    fn visit_block_stmt(&self, statements: Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_class_stmt(&self, name: Token, superclass: Option<Expr>, methods: Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_expression_stmt(&self, expression: Expr) -> Result<R, LoxError>;
    fn visit_function_stmt(&self, name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_if_stmt(&self, condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Result<R, LoxError>;
    fn visit_print_stmt(&self, expression: Expr) -> Result<R, LoxError>;
    fn visit_return_stmt(&self, keyword: Token, value: Option<Expr>) -> Result<R, LoxError>;
    fn visit_var_stmt(&self, name: Token, initializer: Option<Expr>) -> Result<R, LoxError>;
    fn visit_while_stmt(&self, condition: Expr, body: Box<Stmt>) -> Result<R, LoxError>;

    fn execute(&mut self, stmt: Stmt) -> Result<R, LoxError> {
        match stmt {
            Stmt::Block { statements } => self.visit_block_stmt(statements),
            Stmt::Class { name, superclass, methods } => self.visit_class_stmt(name, superclass, methods),
            Stmt::Expression { expression } => self.visit_expression_stmt(expression),
            Stmt::Function { name, params, body } => self.visit_function_stmt(name, params, body),
            Stmt::If { condition, then_branch, else_branch } => self.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::Print { expression } => self.visit_print_stmt(expression),
            Stmt::Return { keyword, value } => self.visit_return_stmt(keyword, value),
            Stmt::Var { name, initializer } => self.visit_var_stmt(name, initializer),
            Stmt::While { condition, body } => self.visit_while_stmt(condition, body),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Block {
        statements: Vec<Stmt>
    },
    Class {
        name: Token,
        // Expr::Variable
        superclass: Option<Expr>,
        // Stmt::Function
        methods: Vec<Stmt>
    },
    Expression {
        expression: Expr
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>
    },
    Print {
        expression: Expr
    },
    Return {
        keyword: Token,
        value: Option<Expr>
    },
    Var {
        name: Token,
        initializer: Option<Expr>
    },
    While {
        condition: Expr,
        body: Box<Stmt>
    }
}