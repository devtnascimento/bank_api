#![allow(dead_code)]

use tokio::io::Result;

#[derive(Debug)]
pub enum Destination {
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
