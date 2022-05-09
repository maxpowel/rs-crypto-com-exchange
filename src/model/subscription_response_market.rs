use serde::{Deserialize};
use crate::model::{TradeResult, CandlestickResult, TickerResult, BookResult};

/// The result of a subscribed event. Identified by the field 'channel'
#[derive(Deserialize, Debug)]
#[serde(tag = "channel")]
pub enum MarketSubscribeResult {

    /// Trade subscription result
    #[serde(rename = "trade")]
    TradeResult(TradeResult),

    /// Candlestick subscription result
    #[serde(rename = "candlestick")]
    CandlestickResult(CandlestickResult),

    /// Ticker subscription result
    #[serde(rename = "ticker")]
    TickerResult(TickerResult),

    /// Book subscription result
    #[serde(rename = "book")]
    BookResult(BookResult),

}

/// Main result of a subscription. It can be the actual result or just the subscription confirmation
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MarketSubscribeResponse {
    /// Confirmation of a subscription
    Confirmation{
        /// The id is the same as the subscription reset sent.
        id: u64,

        /// Code == 0 means ok. Otherwise the code is the error code
        code: u64
    },

    /// New data for the subscription
    Result {
        /// The actual result
        result: MarketSubscribeResult,
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn check_confirmation_structure() {
        let json = "{
            \"id\": 123,
            \"code\": 10
          }";
        let confirmation = from_str::<MarketSubscribeResponse>(json).unwrap();
        match confirmation {
            MarketSubscribeResponse::Confirmation{id, code} => {
                assert_eq!(id, 123);
                assert_eq!(code, 10);
            },
            _ => {
                // Other type, raise error
                assert!(false);
            }
        }
    }


    #[test]
    fn check_result_trade_structure() {
        let json_sub = "{
            \"result\": {\"channel\": \"trade\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"data\": []}
          }";
        let sub = from_str::<MarketSubscribeResponse>(json_sub).unwrap();
        match sub {
            MarketSubscribeResponse::Result{result} => {
                match result {
                    MarketSubscribeResult::TradeResult(result) => {
                        assert_eq!(result.instrument_name, "instrument");
                        assert_eq!(result.subscription, "sub");
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            _ =>{
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_candlestick_structure() {
        let json_sub = "{
            \"result\": {\"channel\": \"candlestick\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"interval\": \"5m\", \"depth\": 100, \"data\": []}
          }";
        let sub = from_str::<MarketSubscribeResponse>(json_sub).unwrap();
        match sub {
            MarketSubscribeResponse::Result{result} => {
                match result {
                    MarketSubscribeResult::CandlestickResult(result) => {
                        assert_eq!(result.instrument_name, "instrument");
                        assert_eq!(result.subscription, "sub");
                        assert_eq!(result.interval, "5m");
                        assert_eq!(result.depth, 100);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            _ =>{
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_ticker_structure() {
        let json_sub = "{
            \"result\": {\"channel\": \"ticker\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"data\": []}
          }";
        let sub = from_str::<MarketSubscribeResponse>(json_sub).unwrap();
        match sub {
            MarketSubscribeResponse::Result{result} => {
                match result {
                    MarketSubscribeResult::TickerResult(result) => {
                        assert_eq!(result.instrument_name, "instrument");
                        assert_eq!(result.subscription, "sub");
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            _ =>{
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_book_structure() {
        let json_sub = "{
            \"result\": {\"channel\": \"book\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"depth\": 123, \"data\": []}
          }";
        let sub = from_str::<MarketSubscribeResponse>(json_sub).unwrap();
        match sub {
            MarketSubscribeResponse::Result{result} => {
                match result {
                    MarketSubscribeResult::BookResult(result) => {
                        assert_eq!(result.instrument_name, "instrument");
                        assert_eq!(result.subscription, "sub");
                        assert_eq!(result.depth, 123);
                    },
                    _ => {
                        assert!(false);
                    }
                }
            },
            _ =>{
                assert!(false);
            }
        }
    }
}