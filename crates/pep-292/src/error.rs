use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;

/// Possible errors returned by [Template::substitute] and
/// [Template::safe_substitute]
///
/// [Template::substitute]: crate::Template::substitute
/// [Template::safe_substitute]: crate::Template::safe_substitute
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError<'input> {
    /// The input is not a valid template string
    ParserError {
        /// Position (in bytes) where the template error was found.
        position: usize,
    },
    /// A key was missing from the substitutions map.
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
