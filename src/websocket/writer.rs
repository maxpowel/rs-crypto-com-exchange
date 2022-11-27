use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::SinkExt;
use crate::model::Request;
use log::{error, debug};

pub async fn writter_task(
    mut writter: futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>,
    mailbox: flume::Receiver<Request>
) {
    loop {
        tokio::select! {
            message = mailbox.recv_async() => {
                match message {
                    Ok(message) => {
                        let text = serde_json::to_string(&message).unwrap();
                        debug!("{}", text);
                        writter.send(Message::text(text)).await.unwrap();
                    },
                    Err(err) => {
                        error!("Error when receiving a message: {}", err)
                    }
                }
            }
        }
    }
}