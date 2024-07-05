use serde::{Serialize, Deserialize};
use serde_aux::prelude::deserialize_number_from_string;


// Main container of the user balance
#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResult {
    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// The actual ticker data
    pub data: Vec<Balance>
}

/// Balance element received from subscription
#[derive(Serialize, Deserialize, Debug)]
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


#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceResult2 {
    /// Subscription name used to subscribe this event
    pub subscription: String,

    /// The actual ticker data
    pub data: Vec<Balance>
}

/// Balance element received from subscription
#[derive(Serialize, Deserialize, Debug)]
pub struct Balance2 {
    /// Balance that user can open new order (Margin Balance - Initial Margin)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_available_balance: f64,

    /// Balance for the margin calculation (Wallet Balance + Unrealized PnL)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_margin_balance: f64,

    /// Total initial margin requirement for all positions and all open orders
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_initial_margin: f64,

    /// Total maintenance margin requirement for all positions
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_maintenance_margin: f64,

    /// Position value in USD
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_position_cost: f64,

    /// Wallet Balance (Deposits - Withdrawals + Realized PnL - Fees)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_cash_balance: f64,

    /// Collateral Value
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_collateral_value: f64,

    /// Current unrealized profit and loss from all open positions (calculated with Mark Price and Avg Price)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_session_unrealized_pnl: f64,

    /// Current realized profit and loss from all open positions (calculated with Mark Price and Avg Price)
    pub instrument_name: String,

    /// Describes whether the account is under liquidation
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_session_realized_pnl: f64,

    /// Describes whether the account is under liquidation
    pub is_liquidating: bool,

    /// The actual leverage used (all open positions combined), i.e. position size / margin balance
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_effective_leverage: f64,

    /// Maximum position size allowed (for all open positions combined)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub position_limit: f64,

    /// Combined position size of all open positions + order exposure on all instruments
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub used_position_limit: f64,

    /// Collateral balances
    pub position_balances: Vec<PositionBalance>
    
}

/// Position balance
#[derive(Serialize, Deserialize, Debug)]
pub struct PositionBalance {

    /// Instrument name of the collateral e.g. USD, CRO, USDT, or DAI
    pub instrument_name: String,

    /// Quantity of the collateral
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub quantity: f64,

    /// Market value of the collateral
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub market_value: f64,

    /// Collateral amount derived by market_value times collateral_weight
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub collateral_amount: f64,

    /// Collateral weight
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub collateral_weight: f64,

     /// Max withdrawal balance of the collateral
     #[serde(deserialize_with = "deserialize_number_from_string")]
     pub max_withdrawal_balance: f64,
}
 
pub fn balance() -> String {
    format!("user.balance")
  }