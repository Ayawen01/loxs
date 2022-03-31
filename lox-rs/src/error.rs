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

impl<'a> Error<'a> {
    pub fn to_string(&self) -> String {
        match self {
            Error::LexError { char, msg, line } => format!("[line {}] LexError `{}` {}", line, char, msg),
            Error::ParseError { msg, line } => format!("[line {}] ParseError {}", line, msg),
            Error::RunTimeError { msg, line } => format!("[line {}] RunTimeError {}", line, msg),
        }
    }
}