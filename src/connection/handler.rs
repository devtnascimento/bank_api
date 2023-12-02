use protocol::message;
use std::error::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::account::Account;

type Err = Box<dyn Error + 'static + Send + Sync>;

pub async fn handle_error(stream: &mut TcpStream, err: Err) -> Err {
    println!("Handle_error call");
    let status = message::Status::Error(err.to_string());

    let resp = match message::serde_json::to_string(&status) {
        Ok(r) => r,
        Err(e) => return Box::new(e),
    };

    if let Err(e) = stream.write_all(resp.as_bytes()).await {
        return Box::new(e);
    }
    err
}

pub async fn handle_register(stream: &mut TcpStream, msg: &str) -> message::Result<()> {
    let account_msg: message::register::Account = message::serde_json::from_str(&msg)?;

    let _ = Account::new(
        account_msg.firs_name,
        account_msg.last_name,
        account_msg.cpf,
        Some(account_msg.pix_key),
    )
    .await?;

    let resp = message::serde_json::to_string(&message::Status::Ok)?;
    stream.write_all(&resp.as_bytes()).await?;

    Ok(())
}
