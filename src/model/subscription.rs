use serde::{Serialize};

/// Parameters of a subscription
#[derive(Serialize, Debug)]
pub struct SubscribeParams {
    /// The channels to subscribe, for example 'user.order.ETH_CRO' 
    pub channels: Vec<String>
}

/// A request done from the client to the exchange
#[derive(Serialize, Debug)]
#[serde(tag = "method")]
pub enum Request {
    /// Heartbeat response that is done every 30 seconds
    #[serde(rename = "public/respond-heartbeat")]
    HeartbeatResponse {
        /// The id has to be the same as the one received by the exchange
        id: u64
    },

    /// Subscription request
    #[serde(rename = "subscribe")]
    Subscribe {
        /// The exchange will response using this id, ideally it is unique
        id: u64,
        /// The actual subscription parameters
        params: SubscribeParams,
        /// Millis since epoch
        nonce: u128,
    },

    /// Authentication request
    #[serde(rename = "public/auth")]
    Auth {
        /// The exchange will response using this id, ideally it is unique
        id: u64,
        /// Client api key
        api_key: String,
        /// Digital signature
        sig: String,
        /// Millis since epoc
        nonce: u128
    },

    /// Subscription request
    #[serde(rename = "subscribe")]
    SignedSubscribe {
        /// The exchange will response using this id, ideally it is unique
        id: u64,
        /// The actual subscription parameters
        params: SubscribeParams,
        /// Millis since epoch
        nonce: u128,

        sig: String,
    },
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::to_string;

    #[test]
    fn check_subscribe_structure() {
        let hb = Request::Subscribe{
            id: 22,
            nonce: 18271187217812782,
            params: SubscribeParams {
                channels: vec!["channel1".into(), "channel2".into()]
            }
        };
        let text = to_string(&hb).unwrap();
        assert_eq!(text, "{\"method\":\"subscribe\",\"id\":22,\"params\":{\"channels\":[\"channel1\",\"channel2\"]},\"nonce\":18271187217812782}");
    }

    #[test]
    fn check_heartbeat_structure() {
        let hb = Request::HeartbeatResponse{id: 19};
        let text = to_string(&hb).unwrap();
        assert_eq!(text, "{\"method\":\"public/respond-heartbeat\",\"id\":19}");
    }

}