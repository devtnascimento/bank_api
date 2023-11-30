use protocol::message::{self, serde_json, Result};
use std::net::SocketAddr;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::account::Account;

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) -> Result<()> {
    println!("Accepted connection from: {}", addr);

    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                let transfer: message::request::Transaction = serde_json::from_str(&msg)?;
            }
            Ok(_) => {
                println!("connection closed by {}: {}", addr, addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
    Ok(())
}
