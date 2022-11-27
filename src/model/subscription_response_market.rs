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


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;


    #[test]
    fn check_result_trade_structure() {
        let json_sub = "{
            \"channel\": \"trade\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"data\": []
          }";
        let res = from_str::<MarketSubscribeResult>(json_sub).unwrap();
        match res {
            MarketSubscribeResult::TradeResult(result) => {
                assert_eq!(result.instrument_name, "instrument");
                assert_eq!(result.subscription, "sub");
                
            },
            _ => {
                assert!(false);
            }
        }

    }

    #[test]
    fn check_result_candlestick_structure() {
        let json_sub = "{
            \"channel\": \"candlestick\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"interval\": \"5m\", \"data\": []
          }";

        let res = from_str::<MarketSubscribeResult>(json_sub).unwrap();
        match res {
            MarketSubscribeResult::CandlestickResult(result) => {
                assert_eq!(result.instrument_name, "instrument");
                assert_eq!(result.subscription, "sub");
                assert_eq!(result.interval, "5m");
            },
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_candlestick_data() {
        let json_sub = "{
            \"channel\": \"candlestick\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"interval\": \"5m\", \"depth\": 100, \"data\": [
                {\"o\":\"3.8349\",\"h\":\"3.8349\",\"l\":\"3.8349\",\"c\":\"3.8349\",\"v\":\"0\",\"t\":1669587780000,\"ut\":1669587780000}
            ]
          }";

        let res = from_str::<MarketSubscribeResult>(json_sub).unwrap();
        match res {
            MarketSubscribeResult::CandlestickResult(result) => {
                assert_eq!(result.instrument_name, "instrument");
                assert_eq!(result.subscription, "sub");
                assert_eq!(result.interval, "5m");
            },
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_ticker_structure() {
        let json_sub = "{
            \"channel\": \"ticker\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"data\": []
          }";
        let res = from_str::<MarketSubscribeResult>(json_sub).unwrap();
        match res {
            MarketSubscribeResult::TickerResult(result) => {
                assert_eq!(result.instrument_name, "instrument");
                assert_eq!(result.subscription, "sub");
            },
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn check_result_book_structure() {
        let json_sub = "{
            \"channel\": \"book\", \"instrument_name\": \"instrument\", \"subscription\": \"sub\", \"depth\": 123, \"data\": []
          }";
        let res = from_str::<MarketSubscribeResult>(json_sub).unwrap();
        
        match res {
            MarketSubscribeResult::BookResult(result) => {
                assert_eq!(result.instrument_name, "instrument");
                assert_eq!(result.subscription, "sub");
                assert_eq!(result.depth, 123);
            },
            _ => {
                assert!(false);
            }
        }
    }
}