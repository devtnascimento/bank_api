mod account;
mod bank;
mod connection;
mod database;
mod io;

use account::Account;
use database::connection::Postgres;
use protocol::message::Result;
use tokio::net::TcpListener;

const BANK_ADDR: &str = "127.0.0.1:8081";
const CB_HOST: &str = "127.0.0.1";
const CB_PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    let first_name = "Thiago".to_string();
    let last_name = "Nascimento".to_string();
    let cpf = "12312312312".to_string();
    let pix_key = "some_cool_key".to_string();

    let db = Postgres::new().await?;
    tokio::spawn(async move { db.conn.await });
    Postgres::init(&db.client);

    let new_account =
        Account::new(BANK_ADDR.to_string(), first_name, last_name, cpf, pix_key).await?;

    println!("{:?}", new_account);

    let listener = TcpListener::bind(BANK_ADDR).await?;

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    Ok(())
}
