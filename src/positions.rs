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

/// Get a open single position by symbol
pub fn get_position(stock_symbol: &str) -> Result<Position, ureq::Error> {
    let address = format!("https://paper-api.alpaca.markets/v2/positions/{stock_symbol}");

    let response = request("GET", &address).call()?;
    let position = response.into_json()?;

    Ok(position)
}

/// Get all open positions
pub fn get_open_positions() -> Result<Vec<Position>, ureq::Error> {
    let address = "https://paper-api.alpaca.markets/v2/positions";

    let response = request("GET", address).call()?;
    let positions = response.into_json()?;

    Ok(positions)
}
