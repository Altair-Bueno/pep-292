use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError<'input> {
    ParserError { position: usize },
    KeyError(&'input str),
}

impl<'input> Display for TemplateError<'input> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateError::ParserError { position } => {
                write!(f, "Invalid placeholder on input at position {position}")
            }
            TemplateError::KeyError(key) => write!(f, "Missing key \"{key}\""),
        }
    }
}

impl Error for TemplateError<'_> {}
