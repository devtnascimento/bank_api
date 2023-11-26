#![allow(dead_code)]

use crate::{
    bank::Destination,
    database::connection::Postgres,
    io::{AccountError, Result},
};
use protocol::{
    message::{self, Register, Request, Response},
    serde_json,
};
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Account {
    bank_name: String,
    pub number: String,
    balance: f64,
    first_name: String,
    last_name: String,
    cpf: String,
    pix_key: String,
}

impl Account {
    pub async fn new(
        bank_name: String,
        first_name: String,
        last_name: String,
        cpf: String,
        pix_key: String,
    ) -> Result<Account> {
        let db = Postgres::new().await?;

        tokio::spawn(async move {
            if let Err(e) = db.conn.await {
                eprintln!("Database connection error: {}", e);
            }
        });
        if let Err(e) = Postgres::init(&db.client).await {
            return Err(Box::new(e));
        }

        let insert_query = format!(
            "INSERT INTO accounts (first_name, last_name, cpf) VALUES ('{}', '{}', '{}') RETURNING id",
            first_name, last_name, cpf
        );

        let account_number: i32 = match db.client.query_one(&insert_query, &[]).await {
            Ok(row) => row.get(0),
            Err(e) => return Err(Box::new(e)),
        };

        Ok(Account {
            bank_name,
            number: account_number.to_string(),
            balance: 0.,
            first_name,
            last_name,
            cpf,
            pix_key,
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
                let pix = message::request::Pix { key };

                let cb_addr = "127.0.0.1";
                let cb_port = "8080";

                let resp = pix.send(cb_addr, cb_port).await?;

                let user = message::User {
                    name: self.first_name.clone(),
                    last_name: self.last_name.clone(),
                    cpf: self.cpf.clone(),
                    pix_key: self.pix_key.clone(),
                };

                let request = message::request::Transaction {
                    bank_name: self.bank_name.clone(),
                    from_user: user,
                    to_user: resp.user,
                    amount: value,
                };
                request.send(cb_addr, cb_port).await?;
                Ok(())
            }
        }
    }
}
