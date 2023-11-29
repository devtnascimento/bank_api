use std::net::SocketAddr;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub async fn handle(mut socket: TcpStream, addr: SocketAddr) {
    println!("Accepted connection from: {}", addr);

    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                let message = String::from_utf8_lossy(&buffer[..n]);
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
}
