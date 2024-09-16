use super::{bars::StockBar, quotes::StockQuote, trades::StockTrade};
use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StockSnapshot {
    pub latest_trade: Option<StockTrade>,
    pub latest_quote: Option<StockQuote>,
    pub minute_bar: Option<StockBar>,
    pub day_bar: Option<StockBar>,
    pub prev_daily_bar: Option<StockBar>,
}

pub type Snapshots = HashMap<String, StockSnapshot>;

pub struct SnapshotsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
    currency: Option<&'a str>,
}

impl<'a> SnapshotsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/snapshots",
            symbols,
            feed: None,
            currency: None,
        }
    }

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    pub fn currency(mut self, currency: &'a str) -> Self {
        self.currency = Some(currency);
        self
    }

    fn build(self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));
        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={}", feed));
        }
        if let Some(currency) = self.currency {
            query.push_str(&format!("&currency={}", currency));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<Snapshots, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: Snapshots = response.into_json()?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot_query() {
        let res = SnapshotsQuery::new(vec!["AAPL"])
            .feed("iex")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL"));
    }
}
