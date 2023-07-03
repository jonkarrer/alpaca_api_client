use super::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TradeActivity {
    pub activity_type: String,
    pub cum_qty: String,
    pub id: String,
    pub leaves_qty: String,
    pub price: String,
    pub qty: String,
    pub side: String,
    pub symbol: String,
    pub transaction_time: String,
    pub order_id: String,
    pub r#type: String,
}

/// Get your recent activity by type
pub fn get_trade_activity_by_type(activity_type: &str) -> Result<Vec<TradeActivity>, ureq::Error> {
    let address = format!(
        "https://paper-api.alpaca.markets/v2/account/activities?activity_type={}",
        activity_type
    );

    let response = request("GET", &address).call()?;
    let activity: Vec<TradeActivity> = response.into_json()?;

    Ok(activity)
}
