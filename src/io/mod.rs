use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum AccountError {
    KeyNotFound(String),
    Other(String),
}

impl Display for AccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AccountError::KeyNotFound(ref key) => write!(f, "Pix key {} not found", key),
            AccountError::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl Error for AccountError {}
