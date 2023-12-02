mod account;
mod bank;
mod connection;
mod database;
mod io;

use database::connection::Postgres;
use protocol::message::Result;
use tokio::net::TcpListener;

const CB_HOST: &str = "127.0.0.1";
const CB_PORT: &str = "8080";

fn get_bank_addr() -> String {
    std::env::var("BANK_ADDR").unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let db = Postgres::new().await?;
    tokio::spawn(async move { db.conn.await });
    Postgres::init(&db.client).await?;

    let addr = get_bank_addr();

    // assert_eq!(addr, String::from("0.0.0.0:9010"));

    let listener = TcpListener::bind(get_bank_addr().as_str()).await?;
    println!("listening at {}...", addr);

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    Ok(())
}
