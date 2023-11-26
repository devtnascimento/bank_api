mod account;
mod bank;
mod connection;
mod database;
mod io;

use account::Account;
use std::error::Error;
use std::result::Result;
use tokio::net::TcpListener;

const BANK_NAME: &str = "bank1";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let first_name = "Thiago".to_string();
    let last_name = "Nascimento".to_string();
    let cpf = "12312312312".to_string();
    let pix_key = "some_cool_key".to_string();

    let new_account =
        match Account::new(BANK_NAME.to_string(), first_name, last_name, cpf, pix_key).await {
            Ok(account) => account,
            Err(e) => panic!("Account Error: {}", e),
        };

    println!("{:?}", new_account);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    Ok(())
}
