#![allow(bare_trait_objects)]
//! Error types for parsing types
use std::error::Error;
use std::fmt;

/// Parsing errors for standard Base64 encoded Binary.
#[derive(Debug, Copy, Clone)]
pub enum ParseBinaryError {
    /// Invalid characters for standard Base64 encoding.
    InvalidEncoding,
    /// Padding is incorrect or not all bytes were properly encoded.
    MissingBytes
}

impl ParseBinaryError {
    fn as_str(&self) -> &str {
        match self {
            ParseBinaryError::InvalidEncoding => {
                "Binary data is encoded with the standard Base64 encoding \
                 ( [a..z] | [A..Z] | + | / | = (padding) )."
            }
            ParseBinaryError::MissingBytes => "Incorrect number of bytes or missing padding."
        }
    }
}

impl fmt::Display for ParseBinaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Error for ParseBinaryError {
    fn description(&self) -> &str {
        self.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

/// Parsing errors for Boolean.
#[derive(Debug)]
pub struct ParseBoolError(());

impl ParseBoolError {
    /// Creates new Boolean parsing error.
    pub(crate) fn new() -> Self {
        ParseBoolError(())
    }
}

impl fmt::Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Provided string was not 'TRUE' or 'FALSE'.")
    }
}

impl Error for ParseBoolError {
    fn description(&self) -> &str {
        "Provided string was not 'TRUE' or 'FALSE'."
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
