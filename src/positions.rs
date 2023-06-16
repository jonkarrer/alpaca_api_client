use super::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Position {
    pub asset_id: String,
    pub symbol: String,
    pub exchange: String,
    pub asset_class: String,
    pub avg_entry_price: String,
    pub qty: String,
    pub qty_available: String,
    pub side: String,
    pub market_value: String,
    pub cost_basis: String,
    pub unrealized_pl: String,
    pub unrealized_plpc: String,
    pub unrealized_intraday_pl: String,
    pub unrealized_intraday_plpc: String,
    pub current_price: String,
    pub lastday_price: String,
    pub change_today: String,
}

pub fn get_position(stock_symbol: &str) -> Position {
    let address = format!("https://paper-api.alpaca.markets/v2/positions/{stock_symbol}");

    let r: Position = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");
    r
}
pub fn get_open_positions() -> Vec<Position> {
    let address = "https://paper-api.alpaca.markets/v2/positions";

    let r: Vec<Position> = request("GET", address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");
    r
}
