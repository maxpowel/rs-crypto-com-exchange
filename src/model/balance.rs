use serde::{Deserialize};

// Main container of the user balance
#[derive(Deserialize, Debug)]
pub struct BalanceResult {
    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// The actual ticker data
    pub data: Vec<Balance>
}

/// Balance element received from subscription
#[derive(Deserialize, Debug)]
pub struct Balance {
    /// Currency name
    pub currency: String,

    /// Total balance
    pub balance: f32,

    /// Total available
    pub available: f32,

    /// Total in any order
    pub order: f32,

    /// Total staked
    pub stake: f32

}

#[cfg(test)]
mod tests {
   
}