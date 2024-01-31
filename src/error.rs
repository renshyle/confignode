use std::{error::Error, fmt::Display};

/// Type of [`ParseError`]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseErrorKind {
    /// A brace was left open or a value at the end of the file was left empty
    UnexpectedEof,
    /// A character was encountered that is invalid in that context
    InvalidCharacter,
}

/// Error type for errors encountered in parsing
#[derive(Debug, PartialEq)]
pub struct ParseError {
    /// The type of parsing error that occurred
    pub kind: ParseErrorKind,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ParseErrorKind::UnexpectedEof => write!(f, "unexpected eof"),
            ParseErrorKind::InvalidCharacter => write!(f, "invalid character"),
        }
    }
}

impl Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
