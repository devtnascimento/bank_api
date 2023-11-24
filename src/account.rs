#![allow(dead_code)]

use crate::{
    bank::Destination,
    database::connection::Postgres,
    io::{AccountError, Result},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    balance: f64,
    first_name: String,
    last_name: String,
    cpf: String,
}

impl Account {
    pub async fn new(first_name: String, last_name: String, cpf: String) -> Result<Account> {
        println!("new account call");
        let db = Postgres::new().await?;
        println!("db connection");

        println!("db init");

        tokio::spawn(async move {
            if let Err(e) = db.conn.await {
                eprintln!("Database connection error: {}", e);
            }
        });
        if let Err(e) = Postgres::init(&db.client).await {
            return Err(Box::new(e));
        }

        println!("tokio::spawn");

        let insert_query = format!(
            "INSERT INTO accounts (first_name, last_name, cpf) VALUES ('{}', '{}', '{}') RETURNING id",
            first_name, last_name, cpf
        );

        let account_number: i32 = match db.client.query_one(&insert_query, &[]).await {
            Ok(row) => row.get(0),
            Err(e) => return Err(Box::new(e)),
        };

        println!("returning from new account");

        Ok(Account {
            number: account_number.to_string(),
            balance: 0.,
            first_name,
            last_name,
            cpf,
        })
    }

    pub async fn transfer(&self, value: f64, destination: Destination) -> Result<()> {
        match destination {
            Destination::AccountNumber(number) => {
                let db = Postgres::new().await?;

                tokio::spawn(async move { db.conn.await });
                if let Err(e) = Postgres::init(&db.client).await {
                    return Err(Box::new(e));
                }
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

                let resp_map: HashMap<String, serde_json::Value> = match resp.status() {
                    reqwest::StatusCode::OK => resp.json().await?,
                    reqwest::StatusCode::NOT_FOUND => {
                        return Err(Box::new(AccountError::KeyNotFound(key)));
                    }
                    _ => {
                        let err_msg = resp.text().await?;
                        return Err(Box::new(AccountError::Other(err_msg)));
                    }
                };

                self.extern_transfer(&resp_map, &value).await?;
                Ok(())
            }
        }
    }

    async fn extern_transfer(
        &self,
        resp_map: &HashMap<String, serde_json::Value>,
        value: &f64,
    ) -> Result<()> {
        let mut data = HashMap::new();

        if let Some(number) = resp_map.get("number") {
            data.insert(number.to_string(), value);
        }

        let url = match resp_map.get("url") {
            Some(url) => url.to_string(),
            None => return Err(Box::new(AccountError::Other("url not found".to_string()))),
        };

        let resp = reqwest::Client::new().post(url).json(&data).send().await?;

        if !resp.status().is_success() {
            return Err(Box::new(AccountError::Other(resp.text().await?)));
        }

        Ok(())
    }
}
