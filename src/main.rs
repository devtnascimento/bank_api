mod account;
mod database;

use account::Account;
// use reqwest;
use std::error::Error;
use std::result::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let first_name = "Thiago".to_string();
    let last_name = "Nascimento".to_string();
    let cpf = "123.123.123.12".to_string();
    let new_account = Account::new(first_name, last_name, cpf).await?;

    println!("NEW ACCOUNT CREATED WITH NUMBER: {}", new_account.number);

    // let resp = reqwest::get("http://127.0.0.1:8080/key")
    //     .await?
    //     .text()
    //     .await?;

    // println!("{:#?}", resp);

    Ok(())
}
