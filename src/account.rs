#![allow(dead_code)]

use crate::{
    bank::Destination, database::connection::Postgres, io::AccountError, BANK_ADDR, CB_HOST,
    CB_PORT,
};
use protocol::{
    message::{self, MessageType, Request, Result, Status},
    serde_json,
};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Account {
    pub number: String,
    balance: f64,
    first_name: String,
    last_name: String,
    cpf: String,
    pix_key: Option<String>,
}

impl Account {
    pub async fn new(
        first_name: String,
        last_name: String,
        cpf: String,
        pix_key: Option<String>,
    ) -> Result<Account> {
        let db = Postgres::new().await?;

        tokio::spawn(async move {
            if let Err(e) = db.conn.await {
                eprintln!("Database connection error: {}", e);
            }
        });

        let insert_query = format!(
            "INSERT INTO accounts (first_name, last_name, cpf) VALUES ('{}', '{}', '{}') RETURNING id",
            first_name, last_name, cpf
        );

        let account_number: i32 = match db.client.query_one(&insert_query, &[]).await {
            Ok(row) => row.get(0),
            Err(e) => return Err(Box::new(e)),
        };

        Ok(Account {
            number: account_number.to_string(),
            balance: 0.,
            first_name,
            last_name,
            cpf,
            pix_key,
        })
    }

    pub async fn from(number: &String) -> Result<Account> {
        let db = Postgres::new().await?;
        let query = "SELECT id, first_name, last_name, cpf, balance FROM accounts WHERE id = $1;";

        let rows = db.client.query(query, &[&number]).await?;

        let first_name: String = rows[0].get("first_name");
        let last_name: String = rows[0].get("last_name");
        let cpf: String = rows[0].get("cpf");
        let balance: f64 = rows[0].get("balance");

        Ok(Account {
            number: number.clone(),
            first_name,
            last_name,
            cpf,
            pix_key: None,
            balance,
        })
    }

    pub async fn transfer(
        &mut self,
        value: f64,
        destination: Destination,
        stream: &mut TcpStream,
    ) -> Result<()> {
        match destination {
            Destination::FromOutside(user) => {
                let db = Postgres::new().await?;

                tokio::spawn(async move { db.conn.await });

                let operations = vec![format!(
                    "UPDATE accounts SET balance = balance + {} WHERE id = {}",
                    value, user.account_number
                )];

                let mut transction = "BEGIN\n".to_string();
                for op in operations {
                    transction = transction + &op + ";\n";
                }

                transction = transction + "COMMIT;\nEND\n";
                db.client.execute(&transction, &[]).await?;

                let resp = message::response::Transaction {
                    status: message::Status::Ok,
                };

                let resp_msg = serde_json::to_string(&resp)?;

                stream.write_all(resp_msg.as_bytes()).await?;
                Ok(())
            }
            Destination::AccountNumber(number) => {
                self.handle_inside_transfer(&value, &number).await?;
                Ok(())
            }
            Destination::PixKey(key) => {
                self.handle_pix_transfer(key, value).await?;
                Ok(())
            }
        }
    }

    async fn handle_inside_transfer(&self, value: &f64, number: &String) -> Result<()> {
        let db = Postgres::new().await?;

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
        db.client.execute(&transction, &[]).await?;

        Ok(())
    }

    async fn handle_outside_transfer(
        &mut self,
        amount: f64,
        from_user: message::User,
        to_user: message::User,
    ) -> Result<()> {
        let db = Postgres::new().await?;

        tokio::spawn(async move { db.conn.await });

        let request = message::request::Transaction {
            from_user,
            to_user: to_user.clone(),
            amount,
        };

        let split_addr: Vec<String> = to_user.bank_addr.split(':').map(String::from).collect();
        let host = &split_addr[0];
        let port = &split_addr[1];

        let resp = request.send(host.as_str(), port.as_str()).await?;

        match resp.status {
            Status::Ok => {
                let operations = vec![format!(
                    "UPDATE accounts SET balance = balance - {} WHERE id = {}",
                    amount, self.number
                )];

                let mut transction = "BEGIN\n".to_string();
                for op in operations {
                    transction = transction + &op + ";\n";
                }
                transction = transction + "COMMIT;\nEND\n";

                db.client.execute(&transction, &[]).await?;

                Ok(())
            }
            Status::Error(e) => Err(Box::new(AccountError::TransferError(e))),
        }
    }

    async fn handle_pix_transfer(&mut self, key: String, amount: f64) -> Result<()> {
        let pix = message::request::Pix {
            message_type: MessageType::Request,
            key,
        };

        let resp = pix.send(CB_HOST, CB_PORT).await?;

        let user = message::User {
            bank_addr: BANK_ADDR.to_string(),
            account_number: self.number.clone(),
            name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            cpf: self.cpf.clone(),
            pix_key: self.pix_key.as_ref().unwrap_or(&"".to_string()).clone(),
        };

        let to_user: message::User;
        if let Some(user_) = resp.user {
            to_user = user_;
        } else {
            return Err(Box::new(AccountError::Other("Destination User not found")));
        }

        self.handle_outside_transfer(amount, user, to_user).await?;

        Ok(())
    }
}
