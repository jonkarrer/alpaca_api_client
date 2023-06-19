use super::request;
use serde::Deserialize;

/// The response object for activities endpoint
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

/// Get trade activity for account
pub fn get_trade_activity() -> Vec<TradeActivity> {
    let address = "https://paper-api.alpaca.markets/v2/account/activities?activity_types=FILL";

    let r: Vec<TradeActivity> = request("GET", address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");
    r
}
