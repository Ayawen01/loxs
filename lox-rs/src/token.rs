use crate::loxtype::LoxType;
use crate::tokentype::TokenType;

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    literal: LoxType,
    line: u16
}