# rs-crypto-com-exchange
This is an unofficial websocket library for the crypto com exchange websocket api https://exchange-docs.crypto.com/spot/index.html#websocket-root-endpoints

Basically, there two kind of clients: `MarketClient` and `UserClient`. The `MarketClient` is used to get global market information for example
for monitoring. The `UserClient` needs authentication and it is used for create orders, check balance and all things related to the user.

This is an example of `MarketClient`
```rust
use tokio::time::{sleep, Duration};
use crypto_com_exchange::{MarketClient, MarketSubscribeResponse, MarketSubscribeResult};


#[tokio::main]
async fn main() {

    let (client, event_receiver) = MarketClient::new().await.unwrap();
    //Documentations says that it is recommented to wait one second before start sending messages
    sleep(Duration::from_secs(1)).await;
    
    // One of every type of message. You can put as many you want. Check the documentation for more details
    let channels = vec![
        "trade.ETH_CRO".to_string(),
        "candlestick.1h.ETH_CRO".to_string(),
        "ticker.ETH_CRO".to_string(),
        "book.ETH_CRO.150".to_string(),
        ];
    println!("Subscribing to {:?}", channels);
    
    client.subscribe(channels).await.unwrap();
    println!("Ready");
    loop {
        match event_receiver.recv_async().await {
            Ok(message) => {
                match message {
                    MarketSubscribeResponse::Confirmation{id, code} => {
                        if code == 0 {
                            println!("Sub {} OK", id);
                        } else {
                            println!("Sub {} fail with code {}", id, code);
                        }

                    },
                    MarketSubscribeResponse::Result{result} => {
                        match result {
                            MarketSubscribeResult::TradeResult(result) => {
                                println!("Trade: {:?}", result);
                                
                            },
                            MarketSubscribeResult::CandlestickResult(result) => {
                                println!("Candlestick: {:?}", result);
                                
                            },
                            MarketSubscribeResult::TickerResult(result) => {
                                println!("Ticker: {:?}", result);
                                
                            },
                            MarketSubscribeResult::BookResult(result) => {
                                println!("Book: {:?}", result);
                                
                            },
                            
                        }
                        

                    }
                }
            },
            Err(err) => {
                println!("Error when receiving a message: {}", err)
            }
        }
    }

}
```

This is an example `UserClient`. It is currently being developed but at least, you can do the authentication and get the balance

```rust
use tokio::time::{sleep, Duration};
use crypto_com_exchange::{UserClient, UserSubscribeResponse, UserSubscribeResult};


#[tokio::main]
async fn main() {

    let (client, event_receiver) = UserClient::new("api_key".to_string(), "api_secret".to_string()).await.unwrap();
    //Documentations says that it is recommented to wait one second before start sending messages
    sleep(Duration::from_secs(1)).await;
    
    // One of every type of message. You can put as many you want. Check the documentation for more details
    let channels = vec![
        "trade.ETH_CRO".to_string(),
        "candlestick.1h.ETH_CRO".to_string(),
        "ticker.ETH_CRO".to_string(),
        "book.ETH_CRO.150".to_string(),
        ];
    println!("Subscribing to {:?}", channels);
    
    client.subscribe(channels).await.unwrap();
    println!("Ready");
    loop {
        match event_receiver.recv_async().await {
            Ok(message) => {
                match message {
                    UserSubscribeResponse::Auth{id, code} => {
                        if code == 0 {
                            println!("Auth {} OK", id);
                        } else {
                            println!("Auth {} fail with code {}", id, code);
                        }
                    }
                    UserSubscribeResponse::Confirmation{id, code} => {
                        if code == 0 {
                            println!("Sub {} OK", id);
                        } else {
                            println!("Sub {} fail with code {}", id, code);
                        }

                    },
                    UserSubscribeResponse::Result{result} => {
                        match result {
                            UserSubscribeResult::BalanceResult(result) => {
                                println!("Trade: {:?}", result);
                                
                            },

                        }
                        

                    }
                }
            },
            Err(err) => {
                println!("Error when receiving a message: {}", err)
            }
        }
    }

}
```