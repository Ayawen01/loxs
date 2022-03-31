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

                b'!' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::Bang_Equal
                    } else {
                        TokenType::Bang
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'=' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::Equal_Equal
                    } else {
                        TokenType::Equal
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'<' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::Less_Equal
                    } else {
                        TokenType::Less
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'>' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::Greater_Equal
                    } else {
                        TokenType::Greater
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'/' => {
                    if self.matching(b'/') {
                        while self.peek() != b'\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        tokens.push(Token{r#type: TokenType::Slash, literal: LoxType::Nil, line: self.line});
                    }
                }

                b' ' |
                b'\r'|
                b'\t' => continue,

                b'\n' => self.line += 1,



                _ => {
                    is_error = true;
                    errors.push(Error::LexError{msg: "未知的词素.", char: byte as char, line: self.line});
                }
            }
        }

        if is_error {
            return Err(errors)
        }

        tokens.push(Token {
            r#type: TokenType::Eof,
            literal: LoxType::Nil,
            line: self.line
        });

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

    #[inline]
    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0'
        }

        *self.source.get(self.current).unwrap()
    }

    fn matching(&mut self, byte: u8) -> bool {
        if self.is_at_end() {
            return false
        }

        if self.peek() != byte {
            return false
        }

        self.current += 1;
        true
    }
}