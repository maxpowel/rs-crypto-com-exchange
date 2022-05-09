use serde::{Deserialize};
use super::BalanceResult;


/// The result of a subscribed event. Identified by the field 'channel'
#[derive(Deserialize, Debug)]
#[serde(tag = "channel")]
pub enum UserSubscribeResult {
    
    /// Trade subscription result
    #[serde(rename = "user.balance")]
    BalanceResult(BalanceResult),
    /*
    /// Candlestick subscription result
    #[serde(rename = "candlestick")]
    CandlestickResult(CandlestickResult),

    /// Ticker subscription result
    #[serde(rename = "ticker")]
    TickerResult(TickerResult),

    /// Book subscription result
    #[serde(rename = "book")]
    BookResult(BookResult),*/

}


/// Main result of an user subscription. It can be the actual result or the auth result
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum UserSubscribeResponse {
    Confirmation{
        /// The id is the same as the subscription reset sent.
        id: u64,

        /// Code == 0 means ok. Otherwise the code is the error code
        code: u64
    },
    
    /// Auth result
    Auth{
        /// The id is the same as the subscription reset sent.
        id: u64,

        /// Code == 0 means ok. Otherwise the code is the error code
        code: u64
    },

    /// New data for the subscription
    Result {
        /// The actual result
        result: UserSubscribeResult,
    }

}

#[cfg(test)]
mod tests {
    
}