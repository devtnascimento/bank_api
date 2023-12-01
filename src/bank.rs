#![allow(dead_code)]

use protocol::message::User;

#[derive(Debug, Clone)]
pub enum Destination {
    FromOutside(User),
    AccountNumber(String),
    PixKey(String),
}

#[derive(Debug, Clone)]
pub enum AccountID {
    Number(String),
    CPF(String),
}
