use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum AccountError<'a> {
    _KeyNotFound(&'a str),
    UserNotFound,
    TransferError(String),
    Other(&'a str),
}

impl<'a> Display for AccountError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AccountError::_KeyNotFound(ref key) => write!(f, "Pix key {} not found", key),
            AccountError::TransferError(ref message) => {
                write!(f, "Could not transfer\nError: {}", message)
            }
            AccountError::UserNotFound => {
                write!(f, "User not found")
            }
            AccountError::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl<'a> Error for AccountError<'a> {}
