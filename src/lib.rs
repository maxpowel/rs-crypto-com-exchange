mod model;
mod client;
mod message;
mod subscription;

pub use model::{Book, BookResult, CandlestickResult, Candlestick, TickerResult, Ticker, Trade, TradeResult, candlestick, TimeFrame, trade, ticker, book, Side, balance, BalanceResult, PositionBalance, Balance};
pub use client::{CryptoClient};
pub use message::SubscribeResult;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
