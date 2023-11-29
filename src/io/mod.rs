use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum AccountError<'a> {
    KeyNotFound(&'a str),
    TransferError(&'a str),
    Other(&'a str),
}

impl<'a> Display for AccountError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AccountError::KeyNotFound(ref key) => write!(f, "Pix key {} not found", key),
            AccountError::TransferError(ref message) => {
                write!(f, "Could not transfer\nError: {}", message)
            }
            AccountError::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl<'a> Error for AccountError<'a> {}
