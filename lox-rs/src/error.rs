use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LoxError {
    LexError {
        char: char,
        msg: Box<str>,
        line: u16
    },
    ParseError {
        msg: Box<str>,
        line: u16
    },
    RunTimeError {
        msg: Box<str>,
        line: u16
    }
}

impl Display for LoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoxError::LexError { char, msg, line } => {
                if *char == ' ' {
                    return write!(f, "[line {}] LexError {}", line, msg)
                }
                write!(f, "[line {}] LexError `{}` {}", line, char, msg)
            }
            LoxError::ParseError { msg, line } => write!(f, "[line {}] ParseError {}", line, msg),
            LoxError::RunTimeError { msg, line } => write!(f, "[line {}] RunTimeError {}", line, msg),
        }
    }
}