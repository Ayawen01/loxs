use crate::{token::Token, error::LoxError};

pub trait VisitorExpr<R> {
    fn visit_assign_expr(&self, name: &Token, value: &Expr) -> Result<R, LoxError>;
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, LoxError>;
    fn visit_call_expr(&self, callee: &Expr, paren: &Token, arguments: &Vec<Expr>) -> Result<R, LoxError>;
    fn visit_get_expr(&self, object: &Expr, name: &Token) -> Result<R, LoxError>;
    fn visit_grouping_expr(&self, expression: &Expr) -> Result<R, LoxError>;
    fn visit_literal_expr(&self, value: &LoxLiteral) -> Result<R, LoxError>;
    fn visit_logical_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, LoxError>;
    fn visit_set_expr(&self, object: &Expr, name: &Token, value: &Expr) -> Result<R, LoxError>;
    fn visit_super_expr(&self, keyword: &Token, methods: &Token) -> Result<R, LoxError>;
    fn visit_this_expr(&self, keyword: &Token) -> Result<R, LoxError>;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<R, LoxError>;
    fn visit_variable_expr(&self, name: &Token);
}

pub trait VisitorStmt<R> {
    fn visit_block_stmt(&self, statements: &Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_class_stmt(&self, name: &Token, superclass: &Option<Expr>, methods: &Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_expression_stmt(&self, expression: &Expr) -> Result<R, LoxError>;
    fn visit_function_stmt(&self, name: &Token, params: &Vec<Token>, body: &Vec<Stmt>) -> Result<R, LoxError>;
    fn visit_if_stmt(&self, condition: &Expr, then_branch: &Stmt, else_branch: &Option<Stmt>) -> Result<R, LoxError>;
    fn visit_print_stmt(&self, expression: &Expr) -> Result<R, LoxError>;
    fn visit_return_stmt(&self, keyword: &Token, value: &Option<Expr>) -> Result<R, LoxError>;
    fn visit_var_stmt(&self, name: &Token, initializer: &Option<Expr>) -> Result<R, LoxError>;
    fn visit_while_stmt(&self, condition: &Expr, body: &Vec<Stmt>) -> Result<R, LoxError>;
}

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

pub enum LoxLiteral {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

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