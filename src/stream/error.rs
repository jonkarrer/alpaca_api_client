use std::fmt;

#[derive(Debug)]
pub enum StreamError {
    /// WebSocket protocol error from tungstenite.
    WebSocket(tungstenite::Error),
    /// JSON deserialization error.
    Json(serde_json::Error),
    /// Authentication failed with the given server message.
    AuthFailed(String),
    /// The connection was closed by the server.
    ConnectionClosed,
}

impl fmt::Display for StreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StreamError::WebSocket(e) => write!(f, "WebSocket error: {}", e),
            StreamError::Json(e) => write!(f, "JSON error: {}", e),
            StreamError::AuthFailed(msg) => write!(f, "Authentication failed: {}", msg),
            StreamError::ConnectionClosed => write!(f, "Connection closed"),
        }
    }
}

impl std::error::Error for StreamError {}

impl From<tungstenite::Error> for StreamError {
    fn from(e: tungstenite::Error) -> Self {
        StreamError::WebSocket(e)
    }
}

impl From<serde_json::Error> for StreamError {
    fn from(e: serde_json::Error) -> Self {
        StreamError::Json(e)
    }
}
