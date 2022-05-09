mod model;
mod websocket;


pub use model::{Book, BookResult, CandlestickResult, Candlestick, TickerResult, Ticker, Trade, TradeResult, UserSubscribeResponse, UserSubscribeResult, MarketSubscribeResponse, MarketSubscribeResult, Request, SubscribeParams, Balance, BalanceResult};
pub use websocket::{MarketClient, UserClient};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
