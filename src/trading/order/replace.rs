use serde::Serialize;

use super::{create::TimeInForce, Order};
use crate::{json_request, trading::AccountType};

#[derive(Serialize, Debug)]
pub struct ReplaceOrderQuery<'a> {
    #[serde(skip_serializing)]
    pub order_id: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<&'a str>,
}

impl<'a> ReplaceOrderQuery<'a> {
    pub fn new(order_id: &'a str) -> Self {
        Self {
            order_id,
            qty: None,
            time_in_force: None,
            limit_price: None,
            stop_price: None,
            trail: None,
            client_order_id: None,
        }
    }

    pub fn qty(mut self, qty: &'a str) -> Self {
        self.qty = Some(qty);
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force.to_string());
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

    pub fn trail(mut self, trail: &'a str) -> Self {
        self.trail = Some(trail);
        self
    }

    pub fn client_order_id(mut self, client_order_id: &'a str) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn send(self, account_type: AccountType) -> Result<Order, ureq::Error> {
        let url = match account_type {
            AccountType::Live => format!("https://api.alpaca.markets/v2/orders/{}", self.order_id),
            AccountType::Paper => format!(
                "https://paper-api.alpaca.markets/v2/orders/{}",
                self.order_id
            ),
        };
        let response = json_request("PATCH", &url)
            .header("Content-Type", "application/json")
            .send_json(&self)?;

        let order = response.into_body().read_json()?;
        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_order() {
        //! Will fail if order is not found or correct type
        let res = ReplaceOrderQuery::new("615bbc4d-966c-470e-bc37-fd0ae3218927")
            .qty("2")
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&res);
        assert!(res.id == "615bbc4d-966c-470e-bc37-fd0ae3218927");
    }
}
