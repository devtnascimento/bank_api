use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    io::Write,
};

#[derive(Debug)]
pub enum AccountError {
    KeyNotFound(String),
    Other(String),
}

impl Display for AccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AccountError::KeyNotFound(key) => write!(f, "Pix key {} not found", key),
            AccountError::Other(message) => write!(f, "{}", message),
        }
    }
}

impl Error for AccountError {}
