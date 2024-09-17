use super::AccountType;
use crate::request;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderQuery<'a> {
    pub symbol: &'a str,
    pub side: String,
    pub r#type: String,
    pub time_in_force: String,
    pub extend_hours: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_percent: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<TakeProfit<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<StopLoss<'a>>,
}

impl<'a> CreateOrderQuery<'a> {
    pub fn new(
        symbol: &'a str,
        side: OrderSide,
        order_type: OrderType,
        time_in_force: TimeInForce,
    ) -> Self {
        Self {
            symbol,
            side: side.to_string(),
            r#type: order_type.to_string(),
            time_in_force: time_in_force.to_string(),
            qty: None,
            notional: None,
            limit_price: None,
            stop_price: None,
            trail_price: None,
            trail_percent: None,
            extend_hours: false,
            client_order_id: None,
            order_class: None,
            take_profit: None,
            stop_loss: None,
        }
    }

    pub fn qty(mut self, qty: &'a str) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn notional(mut self, notional: &'a str) -> Self {
        self.notional = Some(notional);
        self
    }

    pub fn limit_price(mut self, limit_price: &'a str) -> Self {
        self.limit_price = Some(limit_price);
        self
    }

    pub fn stop_price(mut self, stop_price: &'a str) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    pub fn trail_price(mut self, trail_price: &'a str) -> Self {
        self.trail_price = Some(trail_price);
        self
    }

    pub fn trail_percent(mut self, trail_percent: &'a str) -> Self {
        self.trail_percent = Some(trail_percent);
        self
    }

    pub fn extend_hours(mut self, extend_hours: bool) -> Self {
        self.extend_hours = extend_hours;
        self
    }

    pub fn client_order_id(mut self, client_order_id: &'a str) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn order_class(mut self, order_class: OrderClass) -> Self {
        self.order_class = Some(order_class.to_string());
        self
    }

    pub fn take_profit(mut self, take_profit: TakeProfit<'a>) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    pub fn stop_loss(mut self, stop_loss: StopLoss<'a>) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn send(self, account_type: AccountType) -> Result<Order, ureq::Error> {
        let url = match account_type {
            AccountType::Live => "https://api.alpaca.markets/v2/orders",
            AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
        };

        let response = request("POST", url)
            .set("Content-Type", "application/json")
            .send_json(&self)?;

        let order = response.into_json()?;

        Ok(order)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            OrderSide::Buy => "buy".to_string(),
            OrderSide::Sell => "sell".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    StopLimit,
    TrailingStop,
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Market => "market".to_string(),
            OrderType::Limit => "limit".to_string(),
            OrderType::Stop => "stop".to_string(),
            OrderType::StopLimit => "stop_limit".to_string(),
            OrderType::TrailingStop => "trailing_stop".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeInForce {
    Day,
    GoodTilCanceled,
    OpeningOrder,
    ClosingOrder,
    ImmediateOrCancel,
    FillOrKill,
}

impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match self {
            TimeInForce::Day => "day".to_string(),
            TimeInForce::GoodTilCanceled => "gtc".to_string(),
            TimeInForce::OpeningOrder => "opg".to_string(),
            TimeInForce::ClosingOrder => "cls".to_string(),
            TimeInForce::ImmediateOrCancel => "ioc".to_string(),
            TimeInForce::FillOrKill => "fok".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderClass {
    Simple,
    Bracket,
    OneCancelsOther,
    OneTriggersOther,
}

impl ToString for OrderClass {
    fn to_string(&self) -> String {
        match self {
            OrderClass::Simple => "".to_string(),
            OrderClass::Bracket => "bracket".to_string(),
            OrderClass::OneCancelsOther => "oco".to_string(),
            OrderClass::OneTriggersOther => "oto".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TakeProfit<'a> {
    pub limit_price: &'a str,
}

impl<'a> TakeProfit<'a> {
    pub fn new(limit_price: &'a str) -> Self {
        Self { limit_price }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StopLoss<'a> {
    pub stop_price: &'a str,
    pub limit_price: &'a str,
}

impl<'a> StopLoss<'a> {
    pub fn new(stop_price: &'a str, limit_price: &'a str) -> Self {
        Self {
            stop_price,
            limit_price,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order() {
        let order =
            CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::Day)
                .qty("1");

        let res = order.send(AccountType::Paper).unwrap();

        dbg!(&res);

        assert!(false);
    }
}
