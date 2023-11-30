use protocol::message;
use tokio::io::Result;

#[derive(Debug, Clone)]
pub enum Destination {
    FromOutside(message::User),
    AccountNumber(String),
    PixKey(String),
}

#[derive(Debug)]
pub struct User {
    balance: f64,
    key: String,
    account_number: String,
}

#[derive(Debug)]
pub struct Bank {
    users: Vec<User>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { users: Vec::new() }
    }

    pub fn tansfer(value: f64, destiny: Destination) -> Result<()> {
        todo!();
    }
}
