/// Trend enum
#[derive(Debug, PartialEq, Clone)]
pub enum Trend {
    Bullish,
    Bearish,
}

impl ToString for Trend {
    fn to_string(&self) -> String {
        match self {
            Trend::Bullish => "bullish".to_string(),
            Trend::Bearish => "bearish".to_string(),
        }
    }
}
