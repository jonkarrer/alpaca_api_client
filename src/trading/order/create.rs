use super::{Order, OrderSide};
use crate::{json_request, trading::AccountType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderQuery<'a> {
    symbol: &'a str,
    side: String,
    r#type: String,
    time_in_force: String,
    extend_hours: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    notional: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trail_percent: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    take_profit: Option<TakeProfit<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop_loss: Option<StopLoss<'a>>,
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

        let response = json_request("POST", url)
            .header("Content-Type", "application/json")
            .send_json(&self)?;

        let order = response.into_body().read_json()?;
        Ok(order)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    fn test_create_market_order() {
        let order =
            CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::Day)
                .qty("1")
                .send(AccountType::Paper)
                .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_limit_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTilCanceled,
        )
        .limit_price("100")
        .qty("1")
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_stop_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::Stop,
            TimeInForce::GoodTilCanceled,
        )
        .stop_price("100")
        .qty("1")
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_stop_limit_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::StopLimit,
            TimeInForce::GoodTilCanceled,
        )
        .stop_price("100")
        .limit_price("200")
        .qty("1")
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_trailing_stop_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::TrailingStop,
            TimeInForce::GoodTilCanceled,
        )
        .qty("1")
        .trail_percent("10")
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_bracket_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::Market,
            TimeInForce::GoodTilCanceled,
        )
        .qty("1")
        .order_class(OrderClass::Bracket)
        .take_profit(TakeProfit::new("300"))
        .stop_loss(StopLoss::new("200", "199"))
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_oco_order() {
        //! Will fail if a bracket order is not filled yet
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::Limit,
            TimeInForce::GoodTilCanceled,
        )
        .qty("1")
        .order_class(OrderClass::OneCancelsOther)
        .take_profit(TakeProfit::new("199"))
        .stop_loss(StopLoss::new("200", "201"))
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }

    #[test]
    fn test_create_oto_order() {
        let order = CreateOrderQuery::new(
            "AAPL",
            OrderSide::Buy,
            OrderType::Market,
            TimeInForce::GoodTilCanceled,
        )
        .qty("1")
        .order_class(OrderClass::OneTriggersOther)
        .stop_loss(StopLoss::new("200", "189"))
        .send(AccountType::Paper)
        .unwrap();

        dbg!(&order);
        assert!(order.symbol == "AAPL");
    }
}
