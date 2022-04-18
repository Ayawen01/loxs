#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: LoxType,
    pub line: u16
}

#[derive(Debug, Clone)]
pub enum LoxType {
    Id(String),
    String(String),
    Number(f64),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
  
    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
  
    // Literals.
    Identifier, String, Number,
  
    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
  
    Eof
}