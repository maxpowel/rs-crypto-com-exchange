use serde::{Serialize, Deserialize};
use serde_aux::prelude::deserialize_number_from_string;
use chrono::{DateTime, Utc, serde::ts_milliseconds};

// Main container of a trade
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeResult {
    /// Just the instrument name
    pub instrument_name: String,

    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// Actual trades information
    pub data: Vec<Trade>
}

/// Trade element received from subscription
#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    /// Price
    #[serde(rename = "p", deserialize_with = "deserialize_number_from_string")]
    pub price: f64,

    /// Quantity
    #[serde(rename = "q", deserialize_with = "deserialize_number_from_string")]
    pub quantity: f64,

    /// Side, buy or sell (exactly these strings)
    #[serde(rename = "s")]
    pub side: Side,

    /// Transaction id
    #[serde(rename = "d", deserialize_with = "deserialize_number_from_string")]
    pub id: u64,

    /// Time
    #[serde(rename = "t", with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
}

#[derive(Serialize,Deserialize, Debug, PartialEq)]
pub enum Side {
  #[serde(rename = "BUY")]
  Buy,
  #[serde(rename = "SELL")]
  Sell
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
          Side::Buy => write!(f, "BUY"),
          Side::Sell => write!(f, "SELL"),
        }
    }
}

pub fn trade(instrument_name: &str) -> String {
  format!("trade.{instrument_name}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn check_structure() {
        let json = "{
            \"instrument_name\": \"ETH_CRO\",
            \"subscription\": \"trade.ETH_CRO\",
            \"channel\": \"trade\",
            \"data\": [
              {
                \"p\": 162.12,
                \"q\": 11.085,
                \"s\": \"buy\",
                \"d\": 1210447366,
                \"t\": 1587523078844,
                \"dataTime\": 0
              },
              {
                \"p\": 1162.12,
                \"q\": 111.085,
                \"s\": \"sell\",
                \"d\": 11210447366,
                \"t\": 11587523078844,
                \"dataTime\": 0
              }
            ]
          }";
        let ticker_result = from_str::<TradeResult>(json).unwrap();
        assert_eq!(ticker_result.instrument_name, "ETH_CRO");
        assert_eq!(ticker_result.subscription, "trade.ETH_CRO");
        assert_eq!(ticker_result.data.len(), 2);

        // The data
        let data = &ticker_result.data[0];
        assert_eq!(data.price, 162.12);
        assert_eq!(data.quantity, 11.085);
        assert_eq!(data.side, "buy");
        assert_eq!(data.id, 1210447366);
        assert_eq!(data.time, 1587523078844);

        let data2 = &ticker_result.data[1];
        assert_eq!(data2.price, 1162.12);
        assert_eq!(data2.quantity, 111.085);
        assert_eq!(data2.side, "sell");
        assert_eq!(data2.id, 11210447366);
        assert_eq!(data2.time, 11587523078844);
        
    }
}