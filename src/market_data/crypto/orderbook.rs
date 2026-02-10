use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Orderbook {
    pub t: String,
    pub b: Vec<PriceSize>,
    pub a: Vec<PriceSize>,
}

#[derive(Deserialize, Debug)]
pub struct PriceSize {
    pub p: f64,
    pub s: f64,
}

pub type Orderbooks = HashMap<String, Orderbook>;

#[derive(Deserialize, Debug)]
pub struct OrderbookResponse {
    pub orderbooks: Orderbooks,
}

pub struct OrderbookQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
}

impl<'a> OrderbookQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/latest/orderbooks",
            symbols,
        }
    }

    fn build(&self) -> String {
        let symbols = self.symbols.join(",");
        format!("{}?symbols={}", self.url, symbols)
    }

    pub fn send(self) -> Result<Orderbooks, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: OrderbookResponse = response.into_body().read_json()?;
        Ok(response.orderbooks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_orderbook_query() {
        let test_symbol = "BTC/USD";
        let res = OrderbookQuery::new(vec![test_symbol]).send().unwrap();

        dbg!(&res);
        assert!(res.contains_key(test_symbol));
    }
}
