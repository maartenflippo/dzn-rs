use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DznParseError {
    #[error("failed to read from source: {0}")]
    Io(#[from] std::io::Error),
    #[error("failed to decode to UTF-8 string: {0}")]
    InvalidEncoding(#[from] std::str::Utf8Error),
    #[error("failed to parse DZN: expected '{expected}' but got '{actual}'")]
    InvalidSyntax {
        expected: SyntaxElement,
        actual: String,
    },
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum SyntaxElement {
    Identifier,
    Value,
    Equals,
    SemiColon,
}

impl Display for SyntaxElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxElement::Identifier => write!(f, "identifier"),
            SyntaxElement::Value => write!(f, "value"),
            SyntaxElement::Equals => write!(f, "="),
            SyntaxElement::SemiColon => write!(f, ";"),
        }
    }
}
