use tokio::time::Duration;
use tokio::time;
use std::sync::{Arc, Mutex};
use crypto_com_exchange::{trade, CryptoClient, CryptoError, SubscribeResult};
use log::{info, error};
use env_logger::Builder;

// The container is just an object used to share information
// Here I'm just using a dummy counter. In a real application you
// can use it to put a database client for example.
// Put here anything you want to be available in the event_handler
pub struct Container {
    pub count: u64,
}

impl Container {
    fn new() -> Container {
        Container {
            count: 0
        }
    }
}

pub async fn event_handler(event: Result<SubscribeResult, CryptoError>, container: Arc<Mutex<Container>>) {
    match event {
        Ok(message) => {
            match message {
                // Check SubscribeResult struct to see all possible messages here (candlestick, book...)
                SubscribeResult::TradeResult(trade_result) => {
                    info!("New trade: {:?}", trade_result);
                    // Refresh the dummy counter
                    let mut c = container.lock().unwrap();
                    c.count += 1;

                },
                SubscribeResult::UnsubscriptionResult { success } => {
                    info!("Unsubscribe success: {success}");
                },
                _ => {
                    info!("Unexpected")
                }
            }
        },
        Err(error) => {
            error!("{:?}", error);
        }
    }
}

#[tokio::main]
async fn main() {
    Builder::new().parse_filters("info").init();
    info!("Crypto Example");
    let container = Arc::new(Mutex::new(Container::new()));

    let mut crypto_client = CryptoClient::new(event_handler, container.clone());
    info!("Connecting");
    crypto_client.connect_market().await.unwrap();
    info!("Subscribing");
    let instruments = vec!["BTC_USD"];
    crypto_client.subscribe(instruments.into_iter().map(|instrument| trade(instrument)).collect()).await.unwrap();
    info!("Ready");

    // You probably want to run the client while doing other operations. This is a simple example about how to
    // run other stuff while processing crypto messages
    let mut interval = time::interval(Duration::from_millis(5000));
    interval.tick().await;
    let mut listen = true;
    while listen {
        tokio::select!{
            res = crypto_client.wait() => {
                // This is called when for any reason, the client ended
                listen = false;
                if let Err(error) = res {
                    // Some connection error happened o connection was closed.                    
                    error!("{:?}", error)
                } else {
                    // Peacefully exit
                    info!("Peacefully exit");
                }
            },
            _  = interval.tick() => {
                // Dummy counter display
                let c = container.lock().unwrap();
                info!("Total messages: {:?}", c.count)
            }
        }
    }
}
