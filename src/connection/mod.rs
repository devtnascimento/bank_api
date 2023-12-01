mod handler;

use crate::bank::AccountID;
use crate::io::AccountError;
use crate::{account::Account, bank::Destination};
use handler::handle_error;
use protocol::message::{self, serde_json};
use std::net::SocketAddr;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) {
    println!("Accepted connection from: {}", addr);
    let mut buffer = [0; 1024];
    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                let transfer: message::request::Transaction = match serde_json::from_str(&msg) {
                    Ok(transfer) => transfer,
                    Err(err) => {
                        let e = handle_error(&mut socket, Box::new(err)).await;
                        eprintln!("Error: {}", e);
                        break;
                    }
                };

                println!("Received Message: {:#?}", transfer);

                let mut to_account =
                    match Account::from(AccountID::Number(transfer.to_user.account_number.clone()))
                        .await
                    {
                        Ok(account) => account,
                        Err(err) => {
                            let e = handle_error(&mut socket, err).await;
                            eprintln!("Error: {}", e);
                            break;
                        }
                    };

                println!("to_account: {:#?}", to_account);

                if let Err(e) = to_account
                    .transfer(
                        transfer.amount,
                        Destination::FromOutside(transfer.to_user),
                        &mut socket,
                    )
                    .await
                {
                    let e = handle_error(&mut socket, e).await;
                    eprintln!("Error: {}", e);
                    break;
                } else {
                    println!("Sucesso!!!");
                    break;
                }
            }
            Ok(_) => {
                println!("Connection closed by {}: {}", addr, addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}
