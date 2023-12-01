mod account;
mod bank;
mod connection;
mod database;
mod io;

use database::connection::Postgres;
use protocol::message::Result;
use tokio::net::TcpListener;

const BANK_ADDR: &str = "127.0.0.1:9010";
const CB_HOST: &str = "127.0.0.1";
const CB_PORT: &str = "8080";

#[tokio::main]
async fn main() -> Result<()> {
    let db = Postgres::new().await?;
    tokio::spawn(async move { db.conn.await });
    Postgres::init(&db.client).await?;

    let listener = TcpListener::bind(BANK_ADDR).await?;

    while let Ok((socket, addr)) = listener.accept().await {
        tokio::spawn(connection::handle(socket, addr));
    }

    Ok(())
}
