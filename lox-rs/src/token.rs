use crate::loxtype::LoxType;
use crate::tokentype::TokenType;

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: LoxType,
    pub line: u16
}