use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Serialize, Deserialize, Deserializer, Serializer,ser::SerializeTuple, de::{Visitor, SeqAccess}};

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

#[derive(Debug)]
pub struct Offer {
    /// price
    pub price: f64,

    /// Quantity
    pub quantity: f64,

    /// number of orders
    pub amount: i64,
}

/// Convert the struct into the tuple format
impl Serialize for Offer {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
    let mut tup = serializer.serialize_tuple(3)?;
    tup.serialize_element(&self.price)?;
    tup.serialize_element(&self.quantity)?;
    tup.serialize_element(&self.amount)?;
    tup.end()
  }
}

struct OfferVisitor;
/// Convert the tuple into a struct
impl<'de> Visitor<'de> for OfferVisitor {
    type Value = Offer;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a sequence of numbers as strings (price, quantity, amount)")
    }

    fn visit_seq<M>(self, mut seq: M) -> Result<Self::Value, M::Error>
    where
        M: SeqAccess<'de>
    {
        let price: f64 = seq.next_element::<String>().unwrap().unwrap().parse().unwrap();
        let quantity: f64 = seq.next_element::<String>().unwrap().unwrap().parse().unwrap();
        let amount: i64 = seq.next_element::<String>().unwrap().unwrap().parse().unwrap();
        
        Ok(Offer{price, quantity, amount})
    }
}

impl<'de> Deserialize<'de> for Offer {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
    deserializer.deserialize_seq(OfferVisitor)
  }
}

/// Book received from subscription
#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    /// The value is: (price, , )
    /// 
    pub bids: Vec<Offer>,

    /// The value is: (price, quantity, number of Orders)
    pub asks: Vec<Offer>,

    /// The operation time
    #[serde(rename = "t", with = "ts_milliseconds")]
    pub time: DateTime<Utc>,
}

pub fn book(instrument_name: &str, depth: i32) -> String {
  format!("book.{instrument_name}.{depth}")
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
