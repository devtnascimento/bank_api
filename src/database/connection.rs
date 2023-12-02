use tokio_postgres::{tls::NoTlsStream, Client, NoTls, Socket};

use protocol::message::Result;

fn get_db_host() -> String {
    std::env::var("DB_URL").unwrap()
}

pub struct Postgres {
    pub conn: tokio_postgres::Connection<Socket, NoTlsStream>,
    pub client: Client,
}

impl Postgres {
    pub async fn new() -> Result<Postgres> {
        let db_url = get_db_host();
        let (client, conn) = tokio_postgres::connect(db_url.as_str(), NoTls).await?;
        Ok(Postgres { conn, client })
    }

    pub async fn init(client: &Client) -> Result<()> {
        const CREATE_TABLE: &str = r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id SERIAL PRIMARY KEY,
                first_name VARCHAR(50) NOT NULL,
                last_name VARCHAR(50) NOT NULL,
                cpf VARCHAR(11) UNIQUE NOT NULL,
                balance DOUBLE PRECISION CHECK (balance >= 0) DEFAULT 0
            );
        "#;

        match client.execute(CREATE_TABLE, &[]).await {
            Ok(value) => println!("value = {}", value),
            Err(e) => println!("ERROR: {}", e),
        }
        Ok(())
    }
}
