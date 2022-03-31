use crate::{token::Token, tokentype::TokenType, loxtype::LoxType, error::Error};

pub struct Scanner {
    source: Vec<u8>,
    current: usize,
    line: u16
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner { source, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut is_error = false;

        while !self.is_at_end() {
            let byte = self.advance();
            match byte {
                b'(' => tokens.push(Token{r#type: TokenType::Left_Paren,    literal: LoxType::Nil, line: self.line}),
                b')' => tokens.push(Token{r#type: TokenType::Right_Paren,   literal: LoxType::Nil, line: self.line}),
                b'{' => tokens.push(Token{r#type: TokenType::Left_Brace,    literal: LoxType::Nil, line: self.line}),
                b'}' => tokens.push(Token{r#type: TokenType::Right_Brace,   literal: LoxType::Nil, line: self.line}),
                b',' => tokens.push(Token{r#type: TokenType::Comma,         literal: LoxType::Nil, line: self.line}),
                b'.' => tokens.push(Token{r#type: TokenType::Dot,           literal: LoxType::Nil, line: self.line}),
                b'-' => tokens.push(Token{r#type: TokenType::Minus,         literal: LoxType::Nil, line: self.line}),
                b'+' => tokens.push(Token{r#type: TokenType::Plus,          literal: LoxType::Nil, line: self.line}),
                b';' => tokens.push(Token{r#type: TokenType::Semicolon,     literal: LoxType::Nil, line: self.line}),
                b'*' => tokens.push(Token{r#type: TokenType::Star,          literal: LoxType::Nil, line: self.line}),


                _ => {
                    is_error = true;
                    errors.push(Error::LexError{msg: "未知的词素.", char: byte as char, line: self.line});
                }
            }
        }

        if is_error {
            return Err(errors)
        }

        Ok(tokens)
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    #[inline]
    fn advance(&mut self) -> u8 {
        self.current += 1;
        *self.source.get(self.current - 1).unwrap()
    }
}