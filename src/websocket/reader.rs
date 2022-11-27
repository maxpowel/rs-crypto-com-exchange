use futures_util::{StreamExt};
use crate::{model::{Request, MarketMessage, UserMessage, UserSubscribeResponse}, MarketSubscribeResult};
use log::{error, debug};

use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn market_reader_task(
    reader: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    to_writer: flume::Sender<Request>,
    to_client: flume::Sender<MarketSubscribeResult>
) {
    reader.for_each(|message| async {
        let message = message.unwrap();
        match message {
            Message::Text(text) => {
                debug!("{}", text);
                println!("{}", text);
                println!("MESASDFE NUEVO");
                match serde_json::from_str::<MarketMessage>(&text) {
                    Ok(msg) => {
                        println!("COSAAAA {:?}", msg);
                        match msg {
                            MarketMessage::HeartbeatRequest{id} => {                   
                                println!("HEART");
                                to_writer.send_async(Request::HeartbeatResponse{id}).await.unwrap();
                            },
                            MarketMessage::MarketResponse{result} => {
                                println!("OTRO");
                                to_client.send_async(result).await.unwrap();
                            }
                        }
                    }
                    Err(err) => {
                        error!("Error when parsing JSON:\n{}\n{}", text, err);
                    }
                }
            },
            Message::Ping(message) => {
                debug!("PING RECEIVED {:?}", message);
            },
            message => {
                error!("Unexpected message {:?}", message);
            }
        }    
    }).await;
    
}

pub async fn user_reader_task(
    mut reader: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    to_writer: flume::Sender<Request>,
    to_client: flume::Sender<UserSubscribeResponse>
) {
    loop {
        let message = reader.next().await.unwrap().unwrap();
        let text = message.to_text().unwrap();
        debug!("{}", text);
        match serde_json::from_str::<UserMessage>(text) {
            Ok(msg) => {
                match msg {
                    UserMessage::HeartbeatRequest{id} => {                        
                        to_writer.send_async(Request::HeartbeatResponse{id}).await.unwrap();
                    },
                    UserMessage::UserSubscribeResponse(subscribe_response) => {
                        to_client.send_async(subscribe_response).await.unwrap();
                    },
                    UserMessage::AuthResponse{id, code} => {
                        if code != 0 {
                            error!("Auth error with code {}", code);
                        }
                        to_client.send_async(UserSubscribeResponse::Auth{id, code}).await.unwrap();
                    }
                }
            }
            Err(err) => {
                error!("Error when parsing JSON:\n{}\n{}", text, err);
            }
        }
    }
}