use std::{collections::HashMap, error::Error};

use crate::{account, bank::Destination, database::connection::Postgres, io::AccountError};
use tokio_postgres::Error as PostgresError;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    balance: f64,
    first_name: String,
    last_name: String,
    cpf: String,
}

impl Account {
    pub async fn new(
        first_name: String,
        last_name: String,
        cpf: String,
    ) -> Result<Account, PostgresError> {
        let db = Postgres::new().await?;

        if let Err(e) = db.init().await {
            return Err(e);
        }

        tokio::spawn(async move { db.conn.await });

        let insert_query = format!(
            "INSERT INTO accounts (first_name, last_name, cpf) VALUES ('{}', '{}', '{}') RETURNING id",
            first_name, last_name, cpf
        );

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

    pub async fn transfer(
        &self,
        value: f64,
        destination: Destination,
    ) -> Result<(), Box<dyn Error>> {
        match destination {
            Destination::AccountNumber(number) => {
                let db = Postgres::new().await?;

                if let Err(e) = db.init().await {
                    return Err(Box::new(e));
                }

                tokio::spawn(async move { db.conn.await });

                let operations = vec![
                    format!(
                        "UPDATE accounts SET balance = balance + {} WHERE id = {}",
                        value, number
                    ),
                    format!(
                        "UPDATE accounts SET balance = balance - {} WHERE id = {}",
                        value, self.number
                    ),
                ];

                let mut transction = "BEGIN\n".to_string();
                for op in operations {
                    transction = transction + &op + ";\n";
                }

                transction = transction + "COMMIT;\nEND\n";
                if let Err(e) = db.client.execute(&transction, &[]).await {
                    return Err(Box::new(e));
                }

                Ok(())
            }
            Destination::PixKey(key) => {
                let mut params = HashMap::new();

                params.insert("key", &key);

                let resp = reqwest::Client::new()
                    .get("127.0.0.1:8080/key")
                    .query(&params)
                    .send()
                    .await?;

                let account_number = match resp.status() {
                    reqwest::StatusCode::OK => {
                         resp.text().await?
                    }
                    reqwest::StatusCode::NOT_FOUND => {
                        return Err(Box::new(AccountError::KeyNotFound(key)));
                    }
                    _ => {
                        let err_msg = resp.text().await?;
                        return Err(Box::new(AccountError::Other(err_msg)));
                    }
                }




                todo!();
            }
        }
    }
}
