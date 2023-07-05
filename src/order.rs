use super::request;
use serde::{Deserialize, Serialize};
use ureq::json;

/// Possible order statuses
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Accepted,
    AcceptedForBidding,
    Calculated,
    Canceled,
    DoneForDay,
    Expired,
    Filled,
    New,
    PartiallyFilled,
    PendingCancel,
    PendingNew,
    PendingReplace,
    Rejected,
    Replaced,
    Stopped,
    Suspended,
}

/// API object for an Order
#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub id: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub submitted_at: Option<String>,
    pub filled_at: Option<String>,
    pub expired_at: Option<String>,
    pub canceled_at: Option<String>,
    pub failed_at: Option<String>,
    pub replaced_at: Option<String>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: Option<String>,
    pub symbol: String,
    pub asset_class: Option<String>,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: Option<String>,
    pub filled_avg_price: Option<String>,
    pub order_class: Option<String>,
    pub order_type: String,
    pub r#type: String,
    pub side: String,
    pub time_in_force: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<Self>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}

/// Buy or Sell
#[derive(Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Submit basic market order
pub fn place_market_order(
    stock_symbol: &str,
    qty: f32,
    trade_side: OrderSide,
) -> Result<Order, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let side = match trade_side {
        OrderSide::Buy => "buy",
        OrderSide::Sell => "sell",
    };
    let body = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": side,
        "type": "market",
        "time_in_force": "gtc",
    });

    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(body)?;
    let order = response.into_json()?;

    Ok(order)
}

/// Submit a trailing stop order, must have a filled market order for symbol
pub fn place_trailing_stop_order(
    stock_symbol: &str,
    qty: f32,
    trail_percent: f32,
) -> Result<Order, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let body = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": "sell",
        "type": "trailing_stop",
        "time_in_force": "gtc",
        "trail_percent": trail_percent.to_string()
    });

    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(body)?;
    let order = response.into_json()?;

    Ok(order)
}

/// Submit a basic bracket order.
pub fn place_bracket_order(
    stock_symbol: &str,
    qty: f32,
    side: OrderSide,
    take_profit: f32,
    stop_loss: f32,
) -> Result<Order, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let take_profit = (take_profit * 100.0).round() / 100.0;
    let stop_loss = (stop_loss * 100.0).round() / 100.0;
    let stop_limit = ((stop_loss - 1.0) * 100.0).round() / 100.0;

    let order_side = match side {
        OrderSide::Buy => "buy",
        OrderSide::Sell => "sell",
    };

    let body = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": order_side,
        "type": "market",
        "time_in_force": "gtc",
        "order_class": "bracket",
        "take_profit": {
            "limit_price": take_profit.to_string()
        },
        "stop_loss": {
            "stop_price": stop_loss.to_string(),
            "limit_price": stop_limit.to_string()
        }
    });

    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(body)?;
    let order = response.into_json()?;

    Ok(order)
}

/// Submit a One-Triggers-Other for a take profit order
pub fn place_oto_take_profit_order(
    stock_symbol: &str,
    qty: f32,
    take_price: f32,
) -> Result<Order, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let take_profit = (take_price * 100.0).round() / 100.0;

    let body = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": "buy",
        "type": "market",
        "time_in_force": "gtc",
        "order_class": "oto",
        "take_profit": {
            "limit_price": take_profit.to_string()
        },
    });

    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(body)?;
    let order = response.into_json()?;

    Ok(order)
}

/// Submit a One-Triggers-Other for a stop loss order
pub fn place_oto_stop_loss_order(
    stock_symbol: &str,
    qty: f32,
    stop_price: f32,
) -> Result<Order, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let stop_loss = (stop_price * 100.0).round() / 100.0;
    let stop_limit = ((stop_price - 1.0) * 100.0).round() / 100.0;
    let body = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": "buy",
        "type": "market",
        "time_in_force": "gtc",
        "order_class": "oto",
        "stop_loss": {
            "stop_price": stop_loss.to_string(),
            "limit_price": stop_limit.to_string()
        }
    });
    let response = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(body)?;
    let order = response.into_json()?;

    Ok(order)
}

/// Get all orders
pub fn get_orders(query: Option<&str>) -> Result<Vec<Order>, ureq::Error> {
    let url = "https://paper-api.alpaca.markets/v2/orders";
    let address = match query {
        Some(query) => format!("{url}?{query}"),
        _ => format!("{url}"),
    };

    let response = request("GET", &address).call()?;
    let order = response.into_json()?;

    Ok(order)
}

/// Get order by id
pub fn get_order_by_id(order_id: &str) -> Result<Order, ureq::Error> {
    let address = format!("https://paper-api.alpaca.markets/v2/orders/{order_id}");

    let response = request("GET", &address).call()?;
    let order = response.into_json()?;

    Ok(order)
}
