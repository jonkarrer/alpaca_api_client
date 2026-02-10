use super::{AllOrders, Order};
use crate::{request, trading::AccountType};

pub struct GetOrdersQuery<'a> {
    pub url: &'a str,
    pub status: Option<&'a str>,
    pub limit: Option<usize>,
    pub after: Option<&'a str>,
    pub until: Option<&'a str>,
    pub direction: Option<&'a str>,
    pub nested: Option<bool>,
    pub symbols: Option<Vec<&'a str>>,
    pub side: Option<&'a str>,
}

impl<'a> GetOrdersQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/orders",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
            },
            status: None,
            limit: None,
            after: None,
            until: None,
            direction: None,
            nested: None,
            symbols: None,
            side: None,
        }
    }

    pub fn status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn after(mut self, after: &'a str) -> Self {
        self.after = Some(after);
        self
    }

    pub fn until(mut self, until: &'a str) -> Self {
        self.until = Some(until);
        self
    }

    pub fn direction(mut self, direction: &'a str) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn nested(mut self, nested: bool) -> Self {
        self.nested = Some(nested);
        self
    }

    pub fn symbols(mut self, symbols: Vec<&'a str>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    pub fn side(mut self, side: &'a str) -> Self {
        self.side = Some(side);
        self
    }

    fn build(self) -> String {
        let mut query = String::new();
        if let Some(status) = self.status {
            query.push_str(&format!("&status={}", status));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        if let Some(after) = self.after {
            query.push_str(&format!("&after={}", after));
        }
        if let Some(until) = self.until {
            query.push_str(&format!("&until={}", until));
        }
        if let Some(direction) = self.direction {
            query.push_str(&format!("&direction={}", direction));
        }
        if let Some(nested) = self.nested {
            query.push_str(&format!("&nested={}", nested));
        }
        if let Some(symbols) = self.symbols {
            query.push_str(&format!("&symbols={}", symbols.join(",")));
        }
        if let Some(side) = self.side {
            query.push_str(&format!("&side={}", side));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn get_by_id(self, id: &'a str, nested: bool) -> Result<Order, ureq::Error> {
        let route = format!("{}/{}?&nested={}", self.url, id, nested);
        let response = request("GET", &route).call()?;
        let orders: Order = response.into_body().read_json()?;
        Ok(orders)
    }

    pub fn send(self) -> Result<AllOrders, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let orders: AllOrders = response.into_body().read_json()?;
        Ok(orders)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_orders_query() {
        // Query closed orders - result may be empty or contain orders
        let res = GetOrdersQuery::new(AccountType::Paper)
            .status("closed")
            .limit(5)
            .send()
            .unwrap();

        dbg!(&res);
        // Test passes if query succeeds (result can be empty)
        assert!(res.len() <= 5);
    }

    #[test]
    #[ignore] // Requires a valid order ID - run manually with a real order ID
    fn test_get_order_by_id() {
        let res = GetOrdersQuery::new(AccountType::Paper)
            .get_by_id("YOUR_ORDER_ID_HERE", true)
            .unwrap();

        dbg!(&res);
        assert!(!res.id.is_empty());
    }
}
