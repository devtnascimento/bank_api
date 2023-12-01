#![allow(dead_code)]

use protocol::message::User;

#[derive(Debug, Clone)]
pub enum Destination {
    FromOutside(User),
    AccountNumber(String),
    PixKey(String),
}
