mod subscription_response_market;
mod subscription_response_user;
mod subscription;
mod candlestick;
mod book;
mod ticker;
mod trade;
mod balance;


use serde::{Deserialize};
pub use subscription_response_market::MarketSubscribeResult;
pub use subscription_response_user::{UserSubscribeResponse, UserSubscribeResult};
pub use subscription::{Request, SubscribeParams};

pub use book::{BookResult, Book};
pub use candlestick::{CandlestickResult, Candlestick};
pub use ticker::{TickerResult, Ticker};
pub use trade::{TradeResult, Trade};
pub use balance::{BalanceResult, Balance};


///All kind of incoming market messages that the client receive and understand
#[derive(Deserialize, Debug)]
#[serde(tag = "method")]
pub enum MarketMessage {
    /// The exchange is asking for proof of life
    #[serde(rename = "public/heartbeat")]
    HeartbeatRequest{
        /// The same id should be used in the response
        id: u64
    },

    /// A response from a subscription request
    #[serde(rename = "subscribe")]
    MarketResponse{
        result: MarketSubscribeResult
    }
}

///All kind of incoming user messages that the client receive and understand
#[derive(Deserialize, Debug)]
#[serde(tag = "method")]
pub enum UserMessage {
    /// The exchange is asking for proof of life
    #[serde(rename = "public/heartbeat")]
    HeartbeatRequest{
        /// The same id should be used in the response
        id: u64
    },

    /// A response from a subscription request
    #[serde(rename = "subscribe")]
    UserSubscribeResponse(UserSubscribeResponse),

    /// Auth response
    #[serde(rename = "public/auth")]
    AuthResponse {
        /// The id we provided in the auth request
        id: u64,
        /// Auth status code. 0 means ok
        code: u64
    }
}
