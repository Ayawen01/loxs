use crate::token::Token;

pub struct Scanner {
    source: Vec<u8>
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner { source }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        todo!()
    }
}