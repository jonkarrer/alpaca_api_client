use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

use super::error::StreamError;
use super::types::*;

pub(crate) type WsStream = WebSocket<MaybeTlsStream<TcpStream>>;

/// Connect to a WebSocket URL. Returns the live socket.
pub(crate) fn ws_connect(url: &str) -> Result<WsStream, StreamError> {
    let (socket, _response) = connect(url)?;
    Ok(socket)
}

/// Send a JSON message over the socket.
pub(crate) fn ws_send(socket: &mut WsStream, json: &serde_json::Value) -> Result<(), StreamError> {
    let text = serde_json::to_string(json)?;
    socket.send(Message::Text(text))?;
    Ok(())
}

/// Read the next text message from the socket. Handles ping/pong internally.
pub(crate) fn ws_read_text(socket: &mut WsStream) -> Result<String, StreamError> {
    loop {
        let msg = socket.read()?;
        match msg {
            Message::Text(text) => return Ok(text),
            Message::Binary(data) => {
                return String::from_utf8(data.to_vec())
                    .map_err(|_| StreamError::AuthFailed("Invalid binary message".to_string()));
            }
            Message::Ping(data) => {
                socket.send(Message::Pong(data))?;
            }
            Message::Close(_) => return Err(StreamError::ConnectionClosed),
            _ => continue,
        }
    }
}

/// Authenticate on a market data stream.
/// Sends: {"action":"auth","key":"...","secret":"..."}
/// Expects: [{"T":"success","msg":"authenticated"}]
pub(crate) fn auth_market_data(
    socket: &mut WsStream,
    key: &str,
    secret: &str,
) -> Result<(), StreamError> {
    // Read the initial welcome message [{"T":"success","msg":"connected"}]
    let _welcome = ws_read_text(socket)?;

    let auth_msg = serde_json::json!({
        "action": "auth",
        "key": key,
        "secret": secret,
    });
    ws_send(socket, &auth_msg)?;

    let response = ws_read_text(socket)?;
    let parsed: serde_json::Value = serde_json::from_str(&response)?;

    if let Some(arr) = parsed.as_array() {
        if let Some(first) = arr.first() {
            if first.get("T").and_then(|v| v.as_str()) == Some("success")
                && first.get("msg").and_then(|v| v.as_str()) == Some("authenticated")
            {
                return Ok(());
            }
        }
    }
    Err(StreamError::AuthFailed(response))
}

/// Authenticate on the trade updates stream.
/// Sends: {"action":"authenticate","data":{"key_id":"...","secret_key":"..."}}
/// Expects: {"stream":"authorization","data":{"status":"authorized",...}}
pub(crate) fn auth_trade_updates(
    socket: &mut WsStream,
    key: &str,
    secret: &str,
) -> Result<(), StreamError> {
    let auth_msg = serde_json::json!({
        "action": "authenticate",
        "data": {
            "key_id": key,
            "secret_key": secret,
        }
    });
    ws_send(socket, &auth_msg)?;

    let response = ws_read_text(socket)?;
    let parsed: serde_json::Value = serde_json::from_str(&response)?;

    if parsed.get("stream").and_then(|v| v.as_str()) == Some("authorization") {
        if let Some(data) = parsed.get("data") {
            if data.get("status").and_then(|v| v.as_str()) == Some("authorized") {
                return Ok(());
            }
        }
    }
    Err(StreamError::AuthFailed(response))
}

/// Parse a JSON array of market data messages into typed enums.
pub(crate) fn parse_market_data_messages(
    text: &str,
) -> Result<Vec<MarketDataMessage>, StreamError> {
    let arr: Vec<serde_json::Value> = serde_json::from_str(text)?;
    let mut messages = Vec::with_capacity(arr.len());

    for item in arr {
        let msg_type = item.get("T").and_then(|v| v.as_str()).unwrap_or("");
        let msg = match msg_type {
            "t" => MarketDataMessage::Trade(serde_json::from_value(item)?),
            "q" => MarketDataMessage::Quote(serde_json::from_value(item)?),
            "b" => MarketDataMessage::Bar(serde_json::from_value(item)?),
            "d" => MarketDataMessage::DailyBar(serde_json::from_value(item)?),
            "u" => MarketDataMessage::UpdatedBar(serde_json::from_value(item)?),
            "s" => MarketDataMessage::Status(serde_json::from_value(item)?),
            // Control messages: skip
            "success" | "error" | "subscription" => continue,
            _ => continue,
        };
        messages.push(msg);
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_market_data_messages() {
        let json = r#"[{"T":"t","S":"AAPL","i":1,"x":"V","p":126.55,"s":1,"t":"2021-02-22T15:51:44.208Z"},{"T":"b","S":"SPY","o":388.0,"h":389.0,"l":388.0,"c":389.0,"v":100,"t":"2021-02-22T19:15:00Z","n":10,"vw":388.5}]"#;
        let messages = parse_market_data_messages(json).unwrap();
        assert_eq!(messages.len(), 2);
    }

    #[test]
    fn test_parse_control_messages_skipped() {
        let json = r#"[{"T":"success","msg":"connected"}]"#;
        let messages = parse_market_data_messages(json).unwrap();
        assert_eq!(messages.len(), 0);
    }
}
