use serde::{Serialize, Deserialize};

// Main container of a book
#[derive(Serialize, Deserialize, Debug)]
pub struct BookResult {
    /// Just the instrument name
    pub instrument_name: String,

    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// Number of bids and asks to return (up to 150)
    pub depth: i64,

    /// The actual book data
    pub data: Vec<Book>
}

/// TODO maybe create a better structure for the bids and asks. Maybe a struct instead of tuple
/// Book received from subscription
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    /// The value is: (price, quantity, number of Orders)
    pub bids: Vec<(f32, f32, u64)>,

    /// The value is: (price, quantity, number of Orders)
    pub asks: Vec<(f32, f32, u64)>,

    /// The operation time
    #[serde(rename = "t")]
    pub time: u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn check_structure() {
        let json = "{ \"instrument_name\": \"ETH_CRO\",
        \"subscription\": \"book.ETH_CRO.150\",
        \"channel\": \"book\",
        \"depth\": 150,
        \"data\": [
            {
                \"bids\": [
                  [
                    11746.488,
                    128,
                    8
                  ],
                  [
                    22.488,
                    22128.1,
                    228
                  ]
                ],
                \"asks\": [
                  [
                    11747.488,
                    201,
                    12
                  ]
                ],
                \"t\": 1587523078844
            },
            {
                \"bids\": [
                  [
                    11746.488,
                    128,
                    8
                  ]
                ],
                \"asks\": [
                  [
                    11747.488,
                    201,
                    12
                  ]
                ],
                \"t\": 1587523078844
            }
        ]}";
        let book_result = from_str::<BookResult>(json).unwrap();
        assert_eq!(book_result.instrument_name, "ETH_CRO");
        assert_eq!(book_result.depth, 150);
        assert_eq!(book_result.data.len(), 2);
        assert_eq!(book_result.subscription, "book.ETH_CRO.150");
        
        // The data
        let data = &book_result.data[0];
        assert_eq!(data.bids.len(), 2);
        assert_eq!(data.bids[0], (11746.488, 128.0, 8));
        assert_eq!(data.bids[1], (22.488, 22128.1, 228));
        assert_eq!(data.asks.len(), 1);
        assert_eq!(data.asks[0], (11747.488, 201.0, 12));
        assert_eq!(data.time, 1587523078844);
        
    }
}
