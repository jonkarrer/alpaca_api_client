use crate::trading::order::Order;
use serde::Deserialize;

/// Data feed options for stock market data streams.
#[derive(Debug, Clone, Copy)]
pub enum Feed {
    Iex,
    Sip,
    DelayedSip,
    Test,
}

impl ToString for Feed {
    fn to_string(&self) -> String {
        match self {
            Feed::Iex => "iex".to_string(),
            Feed::Sip => "sip".to_string(),
            Feed::DelayedSip => "delayed_sip".to_string(),
            Feed::Test => "test".to_string(),
        }
    }
}

/// A real-time trade message from the stock or crypto stream.
#[derive(Debug, Deserialize, Clone)]
pub struct StreamTrade {
    #[serde(rename = "T")]
    pub msg_type: String,
    #[serde(rename = "S")]
    pub symbol: String,
    pub i: Option<i64>,
    pub x: Option<String>,
    pub p: f64,
    pub s: f64,
    pub t: String,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
}

/// A real-time quote message from the stock or crypto stream.
#[derive(Debug, Deserialize, Clone)]
pub struct StreamQuote {
    #[serde(rename = "T")]
    pub msg_type: String,
    #[serde(rename = "S")]
    pub symbol: String,
    pub bx: Option<String>,
    pub bp: f64,
    pub bs: f64,
    pub ax: Option<String>,
    pub ap: f64,
    #[serde(rename = "as")]
    pub ask_size: f64,
    pub t: String,
    pub c: Option<Vec<String>>,
    pub z: Option<String>,
}

/// A real-time bar message (minute, daily, or updated).
#[derive(Debug, Deserialize, Clone)]
pub struct StreamBar {
    #[serde(rename = "T")]
    pub bar_type: String,
    #[serde(rename = "S")]
    pub symbol: String,
    pub o: f32,
    pub h: f32,
    pub l: f32,
    pub c: f32,
    pub v: u32,
    pub t: String,
    pub n: u32,
    pub vw: f32,
}

/// A trading status message (halts, resumptions, etc.).
#[derive(Debug, Deserialize, Clone)]
pub struct StreamStatus {
    #[serde(rename = "T")]
    pub msg_type: String,
    #[serde(rename = "S")]
    pub symbol: String,
    pub sc: Option<String>,
    pub sm: Option<String>,
    pub rc: Option<String>,
    pub rm: Option<String>,
    pub t: String,
    pub z: Option<String>,
}

/// All possible messages from a market data stream.
#[derive(Debug, Clone)]
pub enum MarketDataMessage {
    Trade(StreamTrade),
    Quote(StreamQuote),
    Bar(StreamBar),
    DailyBar(StreamBar),
    UpdatedBar(StreamBar),
    Status(StreamStatus),
}

/// A trade update message from the account stream.
#[derive(Debug, Deserialize)]
pub struct TradeUpdate {
    pub event: String,
    pub order: Order,
    pub timestamp: Option<String>,
    pub price: Option<String>,
    pub qty: Option<String>,
    pub position_qty: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_stream_trade() {
        let json = r#"{"T":"t","S":"AAPL","i":96921,"x":"V","p":126.55,"s":1,"t":"2021-02-22T15:51:44.208Z","c":["@","I"],"z":"C"}"#;
        let trade: StreamTrade = serde_json::from_str(json).unwrap();
        assert_eq!(trade.symbol, "AAPL");
        assert_eq!(trade.p, 126.55);
    }

    #[test]
    fn test_deserialize_stream_quote() {
        let json = r#"{"T":"q","S":"AMD","bx":"U","bp":87.66,"bs":1,"ax":"Q","ap":87.68,"as":4,"t":"2021-02-22T15:51:45.335Z","c":["R"],"z":"C"}"#;
        let quote: StreamQuote = serde_json::from_str(json).unwrap();
        assert_eq!(quote.symbol, "AMD");
        assert_eq!(quote.bp, 87.66);
    }

    #[test]
    fn test_deserialize_stream_bar() {
        let json = r#"{"T":"b","S":"SPY","o":388.985,"h":389.13,"l":388.975,"c":389.12,"v":49378,"t":"2021-02-22T19:15:00Z","n":797,"vw":389.066}"#;
        let bar: StreamBar = serde_json::from_str(json).unwrap();
        assert_eq!(bar.symbol, "SPY");
        assert_eq!(bar.bar_type, "b");
    }
}
