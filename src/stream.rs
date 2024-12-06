use serde::Deserialize;

// * This is the object that comes from the alpaca websocket for bars
#[allow(dead_code)]
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
