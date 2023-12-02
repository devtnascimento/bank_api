use protocol::message;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const PEAR_ADDR: &str = "127.0.0.1:9020";

#[tokio::test]
async fn register_accounts() -> message::Result<()> {
    let account = message::register::Account {
        firs_name: String::from("Felipe"),
        last_name: String::from("Marques"),
        cpf: String::from("32132132132"),
        pix_key: String::from("felipe.marques@email.com"),
    };

    let req = message::serde_json::to_string(&account)?;
    let mut stream = TcpStream::connect(PEAR_ADDR).await?;
    stream.write_all(req.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;
    let resp = String::from_utf8_lossy(&buffer);
    let xpct_resp = message::serde_json::to_string(&message::Status::Ok)?;

    assert_eq!(resp, xpct_resp);
    Ok(())
}

#[tokio::test]
async fn recv_transfer() -> message::Result<()> {
    let pear_addr = "127.0.0.1:9010";

    let from_user = message::User {
        bank_addr: String::from("127.0.0.1:9020"),
        account_number: String::from("123"),
        name: String::from("foo"),
        last_name: String::from("bar"),
        cpf: String::from("12312312312"),
        pix_key: String::from("some_cool_key"),
    };

    let to_user = message::User {
        bank_addr: pear_addr.to_string(),
        account_number: String::from("1"),
        name: String::from("Thiago"),
        last_name: String::from("Nascimento"),
        cpf: String::from("12312312311"),
        pix_key: String::from("some_coolest_key"),
    };

    let amount = 10.;
    let transaction = message::request::Transaction {
        from_user,
        to_user,
        amount,
    };
    let req = message::serde_json::to_string(&transaction)?;

    let mut stream = TcpStream::connect(&pear_addr).await?;
    stream.write_all(&req.as_bytes()).await?;

    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).await?;

    let resp = String::from_utf8_lossy(&buffer);
    println!("Response = {}", resp);

    let transfer_resp = message::response::Transaction {
        status: message::Status::Ok,
    };

    let xpct_resp = message::serde_json::to_string(&transfer_resp)?;

    assert_eq!(resp, xpct_resp);

    Ok(())
}

#[tokio::test]
async fn send_transfer() -> message::Result<()> {
    todo!();
}
