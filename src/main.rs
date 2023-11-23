mod account;
mod bank;
mod database;
mod io;

use account::Account;
// use reqwest;
use std::error::Error;
use std::result::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let first_name = "Thiago".to_string();
    let last_name = "Nascimento".to_string();
    let cpf = "12312312312".to_string();
    let new_account = Account::new(first_name, last_name, cpf).await?;

    println!("NEW ACCOUNT CREATED WITH NUMBER: {}", new_account.number);

    // println!("{:#?}", resp);

    Ok(())
}
