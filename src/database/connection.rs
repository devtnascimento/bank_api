use tokio_postgres::{tls::NoTlsStream, Client, Error, NoTls, Socket};

pub struct Postgres {
    pub conn: tokio_postgres::Connection<Socket, NoTlsStream>,
    pub client: Client,
}

impl Postgres {
    pub async fn new() -> Result<Postgres, Error> {
        let db_url = "postgres://postgres:123@localhost:5433/postgres";
        let (client, conn) = tokio_postgres::connect(db_url, NoTls).await?;
        Ok(Postgres { conn, client })
    }
    pub async fn init(&self) -> Result<(), Error> {
        const CREATE_TABLE: &str = r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id SERIAL PRIMARY KEY,
                first_name VARCHAR(50) NOT NULL,
                last_name VARCHAR(50) NOT NULL,
                cpf VARCHAR(11) UNIQUE NOT NULL,
                balance numeric DEFAULT 0
            );
        "#;

        if let Err(e) = self.client.execute(CREATE_TABLE, &[]).await {
            return Err(e);
        }
        Ok(())
    }
}