use crate::get_auth;

use super::connection::*;
use super::error::StreamError;
use super::types::*;

/// Builder for a crypto market data WebSocket stream.
///
/// # Example
/// ```no_run
/// use alpaca_api_client::stream::{CryptoStream, MarketDataMessage};
///
/// CryptoStream::new()
///     .subscribe_trades(vec!["BTC/USD", "ETH/USD"])
///     .start(|msg| {
///         match msg {
///             MarketDataMessage::Trade(trade) => {
///                 println!("{}: ${}", trade.symbol, trade.p);
///             }
///             _ => {}
///         }
///     })
///     .unwrap();
/// ```
pub struct CryptoStream {
    trades: Vec<String>,
    quotes: Vec<String>,
    bars: Vec<String>,
    daily_bars: Vec<String>,
    updated_bars: Vec<String>,
}

impl CryptoStream {
    pub fn new() -> Self {
        Self {
            trades: Vec::new(),
            quotes: Vec::new(),
            bars: Vec::new(),
            daily_bars: Vec::new(),
            updated_bars: Vec::new(),
        }
    }

    pub fn subscribe_trades(mut self, symbols: Vec<&str>) -> Self {
        self.trades = symbols.into_iter().map(String::from).collect();
        self
    }

    pub fn subscribe_quotes(mut self, symbols: Vec<&str>) -> Self {
        self.quotes = symbols.into_iter().map(String::from).collect();
        self
    }

    pub fn subscribe_bars(mut self, symbols: Vec<&str>) -> Self {
        self.bars = symbols.into_iter().map(String::from).collect();
        self
    }

    pub fn subscribe_daily_bars(mut self, symbols: Vec<&str>) -> Self {
        self.daily_bars = symbols.into_iter().map(String::from).collect();
        self
    }

    pub fn subscribe_updated_bars(mut self, symbols: Vec<&str>) -> Self {
        self.updated_bars = symbols.into_iter().map(String::from).collect();
        self
    }

    /// Connect, authenticate, subscribe, and begin the message loop.
    /// Calls the provided closure for every received market data message.
    /// This method blocks the calling thread until the connection closes or an error occurs.
    pub fn start<F>(self, mut handler: F) -> Result<(), StreamError>
    where
        F: FnMut(MarketDataMessage),
    {
        let url = "wss://stream.data.alpaca.markets/v1beta3/crypto/us";
        let (key, secret) = get_auth();

        let mut socket = ws_connect(url)?;
        auth_market_data(&mut socket, &key, &secret)?;

        let sub_msg = serde_json::json!({
            "action": "subscribe",
            "trades": self.trades,
            "quotes": self.quotes,
            "bars": self.bars,
            "dailyBars": self.daily_bars,
            "updatedBars": self.updated_bars,
        });
        ws_send(&mut socket, &sub_msg)?;

        // Read subscription confirmation
        let _sub_response = ws_read_text(&mut socket)?;

        // Message loop
        loop {
            let text = ws_read_text(&mut socket)?;
            let messages = parse_market_data_messages(&text)?;
            for msg in messages {
                handler(msg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires API keys and network
    fn test_crypto_stream_connects() {
        let mut count = 0;
        let _ = CryptoStream::new()
            .subscribe_trades(vec!["BTC/USD"])
            .start(|msg| {
                dbg!(&msg);
                count += 1;
                if count >= 3 {
                    panic!("Received 3 messages successfully");
                }
            });
    }
}
