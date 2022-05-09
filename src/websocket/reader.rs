use futures_util::{StreamExt};
use crate::model::{Request, MarketMessage, UserMessage, MarketSubscribeResponse, UserSubscribeResponse};
use log::{error, debug};

pub async fn market_reader_task(
    mut reader: futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
    to_writer: flume::Sender<Request>,
    to_client: flume::Sender<MarketSubscribeResponse>
) {
    loop {
        let message = reader.next().await.unwrap().unwrap();
        let text = message.to_text().unwrap();
        debug!("{}", text);
        match serde_json::from_str::<MarketMessage>(text) {
            Ok(msg) => {
                match msg {
                    MarketMessage::HeartbeatRequest{id} => {                        
                        to_writer.send_async(Request::HeartbeatResponse{id}).await.unwrap();
                    },
                    MarketMessage::MarketSubscribeResponse(subscribe_response) => {
                        to_client.send_async(subscribe_response).await.unwrap();
                    }
                }
            }
            Err(err) => {
                error!("Error when parsing JSON:\n{}\n{}", text, err);
            }
        }
    }
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