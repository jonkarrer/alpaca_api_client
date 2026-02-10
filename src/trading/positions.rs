use super::{order::Order, AccountType};
use crate::request;
use serde::Deserialize;

pub type AllPositions = Vec<Position>;
pub type AllClosedPositions = Vec<ClosedPosition>;

#[derive(Deserialize, Debug)]
pub struct ClosedPosition {
    pub symbol: String,
    pub status: i32,
    pub body: Order,
}

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

pub struct PositionsQuery<'a> {
    url: &'a str,
}

impl<'a> PositionsQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/positions",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/positions",
            },
        }
    }

    pub fn get_all_open_positions(&self) -> Result<AllPositions, ureq::Error> {
        let response = request("GET", self.url).call()?;
        let positions = response.into_body().read_json()?;

        Ok(positions)
    }

    pub fn get_position_by_symbol(&self, symbol: &'a str) -> Result<Position, ureq::Error> {
        let route = format!("{}/{}", self.url, symbol);
        let response = request("GET", &route).call()?;
        let position = response.into_body().read_json()?;

        Ok(position)
    }

    pub fn get_position_by_id(&self, id: &'a str) -> Result<Position, ureq::Error> {
        let route = format!("{}/{}", self.url, id);
        let response = request("GET", &route).call()?;
        let position = response.into_body().read_json()?;

        Ok(position)
    }

    pub fn close_all_positions(
        &self,
        cancel_orders: bool,
    ) -> Result<AllClosedPositions, ureq::Error> {
        let query = format!("?cancel_orders={}", cancel_orders);
        let route = format!("{}{}", self.url, query);
        let response = request("DELETE", &route).call()?;

        let status = response.status().as_u16();
        if status != 200 && status != 207 {
            return Err(ureq::Error::StatusCode(status));
        }

        Ok(response.into_body().read_json()?)
    }

    pub fn close_position_by_id_or_symbol(
        &self,
        id_or_symbol: &'a str,
        qty: Option<f32>,
        percentage: Option<f32>,
    ) -> Result<Order, ureq::Error> {
        let url = format!("{}/{}", self.url, id_or_symbol);
        let mut query = String::new();
        if let Some(qty) = qty {
            query.push_str(&format!("&qty={}", qty));
        }
        if let Some(percentage) = percentage {
            query.push_str(&format!("&percentage={}", percentage));
        }
        let route = format!("{}?{}", url, query);
        let response = request("DELETE", &route).call()?;
        let position = response.into_body().read_json()?;

        Ok(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_positions_query() {
        // Query all open positions - result may be empty if no positions exist
        let res = PositionsQuery::new(AccountType::Paper)
            .get_all_open_positions()
            .unwrap();
        dbg!(&res);
        // Test passes if API call succeeds (result can be empty)
    }

    #[test]
    #[ignore] // Requires an open AAPL position - run manually when you have one
    fn test_get_position_by_symbol() {
        let res = PositionsQuery::new(AccountType::Paper)
            .get_position_by_symbol("AAPL")
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "AAPL");
    }

    #[test]
    #[ignore] // Requires a valid position asset ID - run manually with a real ID
    fn test_get_position_by_id() {
        let res = PositionsQuery::new(AccountType::Paper)
            .get_position_by_id("YOUR_ASSET_ID_HERE")
            .unwrap();
        dbg!(&res);
        assert!(!res.asset_id.is_empty());
    }

    #[test]
    fn test_close_all_positions() {
        // Close all positions - result may be empty if no positions exist
        let res = PositionsQuery::new(AccountType::Paper)
            .close_all_positions(true)
            .unwrap();
        dbg!(&res);
        // Test passes if API call succeeds (result can be empty)
    }

    #[test]
    #[ignore] // Requires an open position - run manually with a real symbol/ID
    fn test_close_position_by_id_or_symbol() {
        let res = PositionsQuery::new(AccountType::Paper)
            .close_position_by_id_or_symbol("AAPL", Some(1.0), None)
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "AAPL");
    }
}
