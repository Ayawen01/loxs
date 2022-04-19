use crate::{token::{Token, LoxType, TokenType}, error::LoxError};

pub struct Scanner {
    source: Vec<u8>,
    current: usize,
    line: u16
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Scanner {
        Scanner { source, current: 0, line: 1 }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<LoxError>> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();
        let mut is_error = false;

        while !self.is_at_end() {
            let byte = self.advance();
            match byte {
                b'(' => tokens.push(Token{r#type: TokenType::LeftParen,    literal: LoxType::Nil, line: self.line}),
                b')' => tokens.push(Token{r#type: TokenType::RightParen,   literal: LoxType::Nil, line: self.line}),
                b'{' => tokens.push(Token{r#type: TokenType::LeftBrace,    literal: LoxType::Nil, line: self.line}),
                b'}' => tokens.push(Token{r#type: TokenType::RightBrace,   literal: LoxType::Nil, line: self.line}),
                b',' => tokens.push(Token{r#type: TokenType::Comma,         literal: LoxType::Nil, line: self.line}),
                b'.' => tokens.push(Token{r#type: TokenType::Dot,           literal: LoxType::Nil, line: self.line}),
                b'-' => tokens.push(Token{r#type: TokenType::Minus,         literal: LoxType::Nil, line: self.line}),
                b'+' => tokens.push(Token{r#type: TokenType::Plus,          literal: LoxType::Nil, line: self.line}),
                b';' => tokens.push(Token{r#type: TokenType::Semicolon,     literal: LoxType::Nil, line: self.line}),
                b'*' => tokens.push(Token{r#type: TokenType::Star,          literal: LoxType::Nil, line: self.line}),

                b'!' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'=' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'<' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    tokens.push(Token{r#type: token_type, literal: LoxType::Nil, line: self.line});
                }
                b'>' => {
                    let token_type = if self.matching(b'=') {
                        TokenType::GreaterEqual
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

                b'"' => {
                    match self.string() {
                        Ok(str) => tokens.push(Token{r#type: TokenType::String, literal: str, line: self.line}),
                        Err(e) => {
                            is_error = true;
                            errors.push(e);
                        }
                    }
                }

                _ => {
                    if self.is_digit(byte) {
                        match self.number() {
                            Ok(num) => tokens.push(Token{r#type: TokenType::Number, literal: num, line: self.line}),
                            Err(e) => {
                                is_error = true;
                                errors.push(e);
                            }
                        }
                    } else if self.is_alpha(byte) {
                        match self.identifier() {
                            Ok((token_type, lox_type)) => {
                                tokens.push(Token{r#type: token_type, literal: lox_type, line: self.line});
                            }
                            Err(e) => {
                                is_error = true;
                                errors.push(e);
                            }
                        }
                    } else {
                        is_error = true;
                        errors.push(LoxError::LexError{msg: "未知的词素.".into(), char: byte as char, line: self.line});
                    }
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

    #[inline]
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return b'\0'
        }

        *self.source.get(self.current + 1).unwrap()
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

    fn string<'a>(&mut self) -> Result<LoxType, LoxError> {
        let start_index = self.current;

        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::LexError{char: ' ', msg: "不是一串完整的字符串.".into(), line: self.line})
        }

        let str = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();

        self.advance();

        Ok(LoxType::String(str))
    }

    fn number<'a>(&mut self) -> Result<LoxType, LoxError> {
        let start_index = self.current - 1;
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            self.advance();
            
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let num = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();
        
        match num.parse::<f64>() {
            Ok(num) => Ok(LoxType::Number(num)),
            Err(_) => Err(LoxError::LexError{char: ' ', msg: "不是一串有效的数字.".into(), line: self.line})
        }
    }

    fn identifier<'a>(&mut self) -> Result<(TokenType, LoxType), LoxError> {
        let start_index = self.current - 1;
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let id = String::from_utf8(self.source[start_index..self.current].to_vec()).unwrap();
        match id.as_str() {
            "and"       =>    Ok((TokenType::And, LoxType::Nil)),
            "class"     =>    Ok((TokenType::Class, LoxType::Nil)),
            "else"      =>    Ok((TokenType::Else, LoxType::Nil)),
            "false"     =>    Ok((TokenType::False, LoxType::Nil)),
            "for"       =>    Ok((TokenType::For, LoxType::Nil)),
            "fun"       =>    Ok((TokenType::Fun, LoxType::Nil)),
            "if"        =>    Ok((TokenType::If, LoxType::Nil)),
            "nil"       =>    Ok((TokenType::Nil, LoxType::Nil)),
            "or"        =>    Ok((TokenType::Or, LoxType::Nil)),
            "print"     =>    Ok((TokenType::Print, LoxType::Nil)),
            "return"    =>    Ok((TokenType::Return, LoxType::Nil)),
            "super"     =>    Ok((TokenType::Super, LoxType::Nil)),
            "this"      =>    Ok((TokenType::This, LoxType::Nil)),
            "true"      =>    Ok((TokenType::True, LoxType::Nil)),
            "var"       =>    Ok((TokenType::Var, LoxType::Nil)),
            "while"     =>    Ok((TokenType::While, LoxType::Nil)),
            _ => {
                Ok((TokenType::Identifier, LoxType::Id(id)))
            }
        }
    }

    #[inline]
    fn is_alpha_numeric(&self, c: u8) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    #[inline]
    fn is_digit(&self, c: u8) -> bool {
        c >= b'0' && c <= b'9'
    }

    #[inline]
    fn is_alpha(&self, c: u8) -> bool {
        c >= b'a' && c <= b'z' ||
        c >= b'A' && c <= b'Z' ||
        c == b'_'
    }
}