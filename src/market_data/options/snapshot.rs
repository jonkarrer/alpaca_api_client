use super::{quotes::OptionQuote, trades::OptionTrade};
use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OptionSnapshot {
    pub greeks: Option<Greeks>,
    pub latest_quote: Option<OptionQuote>,
    pub latest_trade: Option<OptionTrade>,
    pub implied_volatility: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub rho: f64,
    pub theta: f64,
    pub vega: f64,
}

pub type OptionSnapshots = HashMap<String, OptionSnapshot>;

#[derive(Deserialize, Debug)]
pub struct OptionSnapshotResponse {
    pub snapshots: OptionSnapshots,
    pub next_page_token: Option<String>,
}

pub struct OptionSnapshotQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
    updated_since: Option<&'a str>,
    limit: Option<i32>,
}

impl<'a> OptionSnapshotQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/snapshots",
            symbols,
            feed: None,
            updated_since: None,
            limit: None,
        }
    }

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    pub fn updated_since(mut self, updated_since: &'a str) -> Self {
        self.updated_since = Some(updated_since);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    fn build(&self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));
        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={}", feed));
        }
        if let Some(updated_since) = self.updated_since {
            query.push_str(&format!("&updated_since={}", updated_since));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<OptionSnapshots, ureq::Error> {
        let route = self.build();
        let mut snapshots: OptionSnapshots = HashMap::new();
        let mut page_token = None;

        loop {
            // If a token exists, append to address
            let temp_address = match page_token {
                Some(token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };
            let response = request("GET", &temp_address).call()?;
            let response: OptionSnapshotResponse = response.into_json()?;

            // Add snapshots to collection
            for (symbol, snapshot) in response.snapshots {
                snapshots.insert(symbol, snapshot);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_snapshot_query() {
        let res = OptionSnapshotQuery::new(vec!["AAPL241220C00300000"])
            .send()
            .unwrap();
        dbg!(&res);
        assert!(res.contains_key("AAPL241220C00300000"));
    }
}
