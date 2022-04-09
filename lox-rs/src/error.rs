use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Error<'a> {
    LexError {
        char: char,
        msg: &'a str,
        line: u16
    },
    ParseError {
        msg: &'a str,
        line: u16
    },
    RunTimeError {
        msg: &'a str,
        line: u16
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::LexError { char, msg, line } => {
                if *char == ' ' {
                    return write!(f, "[line {}] LexError {}", line, msg)
                }
                write!(f, "[line {}] LexError `{}` {}", line, char, msg)
            }
            Error::ParseError { msg, line } => write!(f, "[line {}] ParseError {}", line, msg),
            Error::RunTimeError { msg, line } => write!(f, "[line {}] RunTimeError {}", line, msg),
        }
    }
}