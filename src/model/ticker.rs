use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Serialize, Deserialize};

// Main container of a ticker
#[derive(Serialize, Deserialize, Debug)]
pub struct TickerResult {
    /// Just the instrument name
    pub instrument_name: String,

    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// The actual ticker data
    pub data: Vec<Ticker>
}

pub fn ticker(instrument_name: &str) -> String {
  format!("ticker.{instrument_name}")
}

/// Ticker element received from subscription
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    /// Price of the 24h highest trade
    #[serde(rename = "h")]
    pub highest: f32,

    /// The total 24h traded volume
    #[serde(rename = "v")]
    pub volume: f32,

    /// The price of the latest trade, null if there weren't any trades
    #[serde(rename = "a")]
    pub latest: f32,

    /// Price of the 24h lowest trade, null if there weren't any trades
    #[serde(rename = "l")]
    pub lowest: f32,

    /// The current best bid price, null if there aren't any bids
    #[serde(rename = "b")]
    pub current: f32,

    /// The current best ask price, null if there aren't any asks
    #[serde(rename = "k")]
    pub best: f32,

    /// 24-hour price change, null if there weren't any trades
    #[serde(rename = "c")]
    pub change: f32,

    /// update time
    #[serde(rename = "t", with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn check_structure() {
        let json = "{
              \"instrument_name\": \"ETH_CRO\",
              \"subscription\": \"ticker.ETH_CRO\",
              \"channel\": \"ticker\",
              \"data\": [
                {
                  \"h\": 1,
                  \"v\": 10232.26315789,
                  \"a\": 173.60263169,
                  \"l\": 0.01,
                  \"b\": 0.02,
                  \"k\": 1.12345680,
                  \"c\": -0.44564773,
                  \"t\": 1587523078844
                }
              ]
            }";
        let ticker_result = from_str::<TickerResult>(json).unwrap();
        assert_eq!(ticker_result.instrument_name, "ETH_CRO");
        assert_eq!(ticker_result.subscription, "ticker.ETH_CRO");
        assert_eq!(ticker_result.data.len(), 1);

        // The data
        let data = &ticker_result.data[0];
        assert_eq!(data.highest, 1.0);
        assert_eq!(data.volume, 10232.26315789);
        assert_eq!(data.latest, 173.60263169);
        assert_eq!(data.lowest, 0.01);
        assert_eq!(data.current, 0.02);
        assert_eq!(data.best, 1.12345680);
        assert_eq!(data.change, -0.44564773);
        assert_eq!(data.time, 1587523078844);
        
    }
}