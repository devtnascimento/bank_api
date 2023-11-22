use crate::database::connection::Postgres;
use tokio_postgres::Error;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    balance: f64,
    first_name: String,
    last_name: String,
    cpf: String,
}

impl Account {
    pub async fn new(first_name: String, last_name: String, cpf: String) -> Result<Account, Error> {
        let db = Postgres::new().await?;

        tokio::spawn(async move { db.conn.await });

        const CREATE_TABLE: &str = r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id SERIAL PRIMARY KEY,
                first_name VARCHAR(50) NOT NULL,
                last_name VARCHAR(50) NOT NULL,
                balance numeric DEFAULT 0
            );
        "#;

        if let Err(e) = db.client.execute(CREATE_TABLE, &[]).await {
            return Err(e);
        } else {
            println!("accounts table has been created");
        }

        let insert_query = format!(
            "INSERT INTO accounts (first_name, last_name) VALUES ('{}', '{}') RETURNING id",
            first_name, last_name
        );

        println!("insert_query = {}", insert_query);

        let account_number: i32 = match db.client.query_one(&insert_query, &[]).await {
            Ok(row) => row.get(0),
            Err(e) => return Err(e),
        };

        Ok(Account {
            number: account_number.to_string(),
            balance: 0.,
            first_name,
            last_name,
            cpf,
        })
    }
}
