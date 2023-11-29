use crate::{
    bank::{Destination, User},
    database::connection::Postgres,
    io::AccountError,
    BANK_ADDR, CB_HOST, CB_PORT,
};
use protocol::{
    message::{self, MessageType, Register, Request, Response, Result, Status},
    serde_json,
};
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
                self.handle_inside_transfer(&value, &number).await?;
                Ok(())
            }
            Destination::PixKey(key) => {
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
                    pix_key: self.pix_key.clone(),
                };

                let to_user: message::User;
                if let Some(user_) = resp.user {
                    to_user = user_;
                } else {
                    return Err(Box::new(AccountError::Other("Destination User not found")));
                }

                let request = message::request::Transaction {
                    bank_addr: BANK_ADDR.to_string(),
                    from_user: user,
                    to_user,
                    amount: value,
                };
                request.send(CB_HOST, CB_PORT).await?;
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
            Status::Ok => {todo!();}
            Status::Error(e) => {
                return Err(Box::new(AccountError::TransferError(e.as_str())));
            }
        }

        let err_msg = match &resp.status {
            Stat
        };

        if let Status::Error(e) = resp.status {
            err_msg = e.clone().as_str();
        }

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

    async fn handle_pix_transfer(&self, key: String, amount: f64) -> Result<()> {
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
            pix_key: self.pix_key.clone(),
        };

        let to_user: message::User;
        if let Some(user_) = resp.user {
            to_user = user_;
        } else {
            return Err(Box::new(AccountError::Other("Destination User not found")));
        }

        // abrir um socket

        self.handle_outside_transfer(amount, user, to_user).await?;

        Ok(())
    }
}
