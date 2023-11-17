tokio::io::Result;
use super::bank::{
    Bank,
    Destination
};

#[derive(Debug)]
pub struct CentralBank {
    banks: Vec<Bank>,
}

impl CentralBank {
    pub fn new() -> CentralBank {
        CentralBank {
            banks: Vec::new()
        }
    }

    pub fn get_account(destination: Destination) -> String {
        todo!();
    }
}

