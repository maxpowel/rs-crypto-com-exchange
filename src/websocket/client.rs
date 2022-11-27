use tokio_tungstenite::{connect_async};
use futures_util::{StreamExt};
use crate::MarketSubscribeResult;

use super::{reader::user_reader_task, reader::market_reader_task, writer::writter_task};
use super::super::model::{Request, SubscribeParams, UserSubscribeResponse};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{sleep, Duration};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::error;

fn nonce() -> u128 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_millis()
}


pub struct MarketClient {
    handles: Vec<tokio::task::JoinHandle<()>>,
    to_writer: flume::Sender<Request>,

}


impl MarketClient {
    pub async fn new() -> Result<(MarketClient, flume::Receiver<MarketSubscribeResult>), tokio_tungstenite::tungstenite::Error> {


        match connect_async("wss://stream.crypto.com/v2/market").await {
            Ok(connection) => {
                let (ws_stream, _) = connection;
                let (write, read) = ws_stream.split();
                let (to_writer, writer_mailbox) = flume::unbounded();
                let (to_client, client_mailbox) = flume::unbounded();

                Ok((MarketClient {
                    to_writer: to_writer.clone(),
                    handles: vec![
                        tokio::spawn(market_reader_task(read, to_writer, to_client)),
                        tokio::spawn(writter_task(write, writer_mailbox))
                    ],
                }, client_mailbox))
            },
            Err(err) => {
                Err(err)
            }
        }
        
    }

    pub async fn subscribe(&self, channels: Vec<String>) ->Result<(), flume::SendError<Request>> {
        self.to_writer.send_async(Request::Subscribe{
            id: 1,
            params: SubscribeParams{channels},
            nonce: nonce()
        }).await
    }

    pub async fn join(self) {
        futures::future::join_all(self.handles).await;
    }
}


type HmacSha256 = Hmac<Sha256>;


pub struct UserClient {
    handles: Vec<tokio::task::JoinHandle<()>>,
    to_writer: flume::Sender<Request>,
    api_key: String,
    api_secret: String,
}


impl UserClient {
    pub async fn new(api_key: String, api_secret: String) -> Result<(UserClient, flume::Receiver<UserSubscribeResponse>), Box<dyn error::Error>> {

        match connect_async("wss://stream.crypto.com/v2/user").await {
            Ok(connection) => {
                let (ws_stream, _) = connection;
                let (write, read) = ws_stream.split();
                let (to_writer, writer_mailbox) = flume::unbounded();
                let (to_client, client_mailbox) = flume::unbounded();
                let client = UserClient {
                    to_writer: to_writer.clone(),
                    handles: vec![
                        tokio::spawn(user_reader_task(read, to_writer, to_client)),
                        tokio::spawn(writter_task(write, writer_mailbox))
                    ],
                    api_key: api_key,
                    api_secret: api_secret,
                };
                
                sleep(Duration::from_secs(1)).await;
                match client.auth().await {
                    Ok(_) => Ok((client, client_mailbox)),
                    Err(err) => Err(err.into())
                }
            },
            Err(err) => {
                Err(err.into())
            }
        }
        
    }

    pub async fn subscribe(&self, channels: Vec<String>) ->Result<(), flume::SendError<Request>> {
        self.to_writer.send_async(Request::Subscribe{
            id: 1,
            params: SubscribeParams{channels},
            nonce: nonce()
        }).await
    }

    async fn auth(&self) ->Result<(), flume::SendError<Request>> {
        let n = nonce();
        let id = 123;
        let message_to_sig = vec!["public/auth".into(), id.to_string(), self.api_key.to_owned(), n.to_string()].concat();
        let mut mac = HmacSha256::new_from_slice(self.api_secret.as_bytes()).unwrap();
        mac.update(message_to_sig.as_bytes());
        let result = mac.finalize();
        let f = result.into_bytes();

        self.to_writer.send_async(Request::Auth{
            id: id,
            api_key: self.api_key.to_owned(),
            sig: hex::encode(f),
            nonce: n
        }).await

    }


    pub async fn join(self) {
        futures::future::join_all(self.handles).await;
    }
}