mod account;
mod bank;
mod connection;
mod database;
mod io;
mod messages;

use account::Account;
// use reqwest;
use std::error::Error;
use std::result::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    let first_name = "Thiago".to_string();
    let last_name = "Nascimento".to_string();
    let cpf = "12312312312".to_string();

    let new_account: Account;

    match Account::new(first_name, last_name, cpf).await {
        Ok(account) => new_account = account,
        Err(e) => println!("Account Error: {}", e),
    };

    // println!("{:#?}", resp);

    Ok(())
}
