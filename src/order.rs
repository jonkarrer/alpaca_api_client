use super::request;
use serde::{Deserialize, Serialize};
use serde_json::*;

/// The current status of the order in its lifecycle
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

#[derive(Deserialize, Debug)]
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

#[derive(Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub fn place_market_order(stock_symbol: &str, qty: f32, trade_side: OrderSide) -> Order {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let side = match trade_side {
        OrderSide::Buy => "buy",
        OrderSide::Sell => "sell",
    };
    let order = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": side,
        "type": "market",
        "time_in_force": "gtc",
    });

    let r = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(order)
        .expect("Failed To Place Order");
    let res: Option<Order> = r.into_json().expect("Failed To Convert Order To JSON");
    res.expect("No Order In Response")
}

pub fn place_trailing_stop_order(stock_symbol: &str, qty: f32, trail_percent: f32) -> Order {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let order = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": "sell",
        "type": "trailing_stop",
        "time_in_force": "gtc",
        "trail_percent": trail_percent.to_string()
    });

    let err_message = format!("Failed To Place Order {}", stock_symbol);
    let r = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(order)
        .expect(&err_message);
    let res: Option<Order> = r.into_json().expect("Failed To Convert Order To JSON");
    res.expect("No Order In Response")
}

pub fn place_bracket_order(
    stock_symbol: &str,
    qty: f32,
    side: &str,
    take_profit: f32,
    stop_loss: f32,
) -> Order {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let take_profit = (take_profit * 100.0).round() / 100.0;
    let stop_loss = (stop_loss * 100.0).round() / 100.0;
    let stop_limit = ((stop_loss - 1.0) * 100.0).round() / 100.0;

    let bracket_order = json!({
        "symbol": stock_symbol,
        "qty": qty.to_string(),
        "side": side,
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

    let r = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(bracket_order)
        .expect("Failed To Place Order");
    let res: Option<Order> = r.into_json().expect("Failed To Convert Order To JSON");
    res.expect("No Order In Response")
}

pub fn place_oto_take_profit_order(stock_symbol: &str, qty: f32, take_price: f32) -> Order {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let take_profit = (take_price * 100.0).round() / 100.0;

    let oto_order = json!({
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

    let err_message = format!("Failed To Place Take Profit Order {}", stock_symbol);
    let r = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(oto_order)
        .expect(&err_message);
    let res: Option<Order> = r
        .into_json()
        .expect("Failed To Convert Take Profit Order To JSON");

    res.expect("No Order Object In Response")
}

pub fn place_oto_stop_loss_order(stock_symbol: &str, qty: f32, stop_price: f32) -> Order {
    let url = "https://paper-api.alpaca.markets/v2/orders";

    let stop_loss = (stop_price * 100.0).round() / 100.0;
    let stop_limit = ((stop_price - 1.0) * 100.0).round() / 100.0;
    let bracket_order = json!({
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

    let err_message = format!("Failed To Place Stop Loss Order {}", stock_symbol);
    let r = request("POST", url)
        .set("Content-Type", "application/json")
        .send_json(bracket_order)
        .expect(&err_message);
    let res: Option<Order> = r
        .into_json()
        .expect("Failed To Convert Stop Loss Order To JSON");

    res.expect("No Order Object In Response")
}
pub fn get_orders(query: Option<&str>) -> Vec<Order> {
    let url = "https://paper-api.alpaca.markets/v2/orders";
    let address = match query {
        Some(query) => format!("{url}?{query}"),
        _ => format!("{url}"),
    };

    request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json")
}

pub fn get_order_by_id(order_id: &str) -> Order {
    let address = format!("https://paper-api.alpaca.markets/v2/orders/{order_id}");

    request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json")
}
