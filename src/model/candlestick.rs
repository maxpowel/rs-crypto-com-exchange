use serde::{Deserialize};

// Main container of a candlestick
#[derive(Deserialize, Debug)]
pub struct CandlestickResult {
    /// Just the instrument name
    pub instrument_name: String,

    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// TODO use an enum for that
    /// The time interval affected by this candlestick. The options are
    /// 1m : one minute
    /// 5m : five minutes
    /// 15m : 15 minutes
    /// 30m: 30 minutes
    /// 1h : one hour
    /// 4h : 4 hours
    /// 6h : 6 hours
    /// 12h : 12 hours
    /// 1D : one day
    /// 7D : one week
    /// 14D : two weeks
    /// 1M : one month
    pub interval: String,

    /// Undocumented value
    pub depth: u64,

    /// Actual candlestick information
    pub data: Vec<Candlestick>
}

/// Candlestick received from subscription
#[derive(Deserialize, Debug)]
pub struct Candlestick {

    /// Open price
    #[serde(rename = "o")]
    pub open: f32,
    
    /// Close price
    #[serde(rename = "c")]
    pub close: f32,

    /// Highest price
    #[serde(rename = "h")]
    pub high: f32,

    /// Lowest price
    #[serde(rename = "l")]
    pub low: f32,

    /// Volume
    #[serde(rename = "v")]
    pub volume: f32,

    /// When the candlestick starts
    #[serde(rename = "t")]
    pub start_time: u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn check_structure() {
        let json = "{
            \"instrument_name\": \"ETH_CRO\",
            \"subscription\": \"candlestick.1m.ETH_CRO\",
            \"channel\": \"candlestick\",
            \"depth\":300,
            \"interval\": \"1m\",
            \"data\":[
              {
                \"o\": 162.03,
                \"c\": 162.04,
                \"h\": 161.96,
                \"l\": 161.98,
                \"v\": 336.452694,
                \"t\": 1589441241
              },
              {
                \"o\": 163.03,
                \"c\": 163.04,
                \"h\": 162.96,
                \"l\": 162.98,
                \"v\": 336.452694,
                \"t\": 1589443241
              }
              ]
          }";
        //let json = "{\"instrument_name\":\"ETH_CRO\",\"subscription\":\"candlestick.5m.ETH_CRO\",\"channel\":\"candlestick\",\"depth\":300,\"interval\":\"5m\",\"data\":[{\"t\":1648065300000,\"o\":6962.37,\"h\":6988.77,\"l\":6951.97,\"c\":6962.33,\"v\":0.00662}]}";
        let candlestick_result = from_str::<CandlestickResult>(json).unwrap();
        assert_eq!(candlestick_result.instrument_name, "ETH_CRO");
        assert_eq!(candlestick_result.subscription, "candlestick.1m.ETH_CRO");
        assert_eq!(candlestick_result.interval, "1m");
        assert_eq!(candlestick_result.depth, 300);
        assert_eq!(candlestick_result.data.len(), 2);

        // The data
        let data = &candlestick_result.data[0];
        assert_eq!(data.open, 162.03);
        assert_eq!(data.close, 162.04);
        assert_eq!(data.high, 161.96);
        assert_eq!(data.low, 161.98);
        assert_eq!(data.volume, 336.452694);
        assert_eq!(data.start_time, 1589441241);
        
    }
}