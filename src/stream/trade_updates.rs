use crate::get_auth;
use crate::trading::AccountType;

use super::connection::*;
use super::error::StreamError;
use super::types::TradeUpdate;

/// Builder for the trade updates (account events) WebSocket stream.
///
/// # Example
/// ```no_run
/// use alpaca_api_client::stream::TradeUpdateStream;
/// use alpaca_api_client::trading::AccountType;
///
/// TradeUpdateStream::new(AccountType::Paper)
///     .start(|update| {
///         println!("[{}] {} - {}", update.event, update.order.symbol, update.order.side);
///     })
///     .unwrap();
/// ```
pub struct TradeUpdateStream {
    account_type: AccountType,
}

impl TradeUpdateStream {
    pub fn new(account_type: AccountType) -> Self {
        Self { account_type }
    }

    /// Connect, authenticate, subscribe to trade_updates, and begin the message loop.
    /// Calls the provided closure for every trade update event.
    /// This method blocks the calling thread until the connection closes or an error occurs.
    pub fn start<F>(self, mut handler: F) -> Result<(), StreamError>
    where
        F: FnMut(TradeUpdate),
    {
        let url = match self.account_type {
            AccountType::Paper => "wss://paper-api.alpaca.markets/stream",
            AccountType::Live => "wss://api.alpaca.markets/stream",
        };
        let (key, secret) = get_auth();

        let mut socket = ws_connect(url)?;
        auth_trade_updates(&mut socket, &key, &secret)?;

        let listen_msg = serde_json::json!({
            "action": "listen",
            "data": {
                "streams": ["trade_updates"]
            }
        });
        ws_send(&mut socket, &listen_msg)?;

        // Message loop
        loop {
            let text = ws_read_text(&mut socket)?;
            let parsed: serde_json::Value = serde_json::from_str(&text)?;

            if parsed.get("stream").and_then(|v| v.as_str()) == Some("trade_updates") {
                if let Some(data) = parsed.get("data") {
                    let update: TradeUpdate = serde_json::from_value(data.clone())?;
                    handler(update);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requires API keys, network, and active orders
    fn test_trade_update_stream_connects() {
        let _ = TradeUpdateStream::new(AccountType::Paper).start(|update| {
            dbg!(&update);
        });
    }
}
