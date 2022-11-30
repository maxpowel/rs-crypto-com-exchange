mod model;
mod transport;
mod message;
mod subscription;

pub use model::{Book, BookResult, CandlestickResult, Candlestick, TickerResult, Ticker, Trade, TradeResult, Balance, BalanceResult};
pub use transport::{CryptoTransport};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
