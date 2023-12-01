use protocol::message;
use std::error::Error;
use tokio::{io::AsyncWriteExt, net::TcpStream};

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
