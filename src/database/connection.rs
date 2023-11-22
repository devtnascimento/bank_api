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
}
