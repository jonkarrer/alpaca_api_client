use super::{quotes::CryptoQuote, trades::CryptoTrade};
use crate::{market_data::stocks::bars::StockBar, request};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CryptoSnapshot {
    pub daily_bar: Option<StockBar>,
    pub minute_bar: Option<StockBar>,
    pub prev_daily_bar: Option<StockBar>,
    pub latest_trade: Option<CryptoTrade>,
    pub latest_quote: Option<CryptoQuote>,
}

pub type CryptoSnapshots = HashMap<String, CryptoSnapshot>;

#[derive(Deserialize, Debug)]
pub struct CryptoSnapshotsResponse {
    pub snapshots: CryptoSnapshots,
}

pub struct SnapshotsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
}

impl<'a> SnapshotsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/snapshots",
            symbols,
        }
    }

    fn build(self) -> String {
        format!("{}?symbols={}", self.url, self.symbols.join(","))
    }

    pub fn send(self) -> Result<CryptoSnapshots, ureq::Error> {
        let route = self.build();
        dbg!(&route);
        let response = request("GET", &route).call()?;
        let response: CryptoSnapshotsResponse = response.into_json()?;
        Ok(response.snapshots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_snapshots_query() {
        let symbols = vec!["BTC/USD"];
        let query = SnapshotsQuery::new(symbols).send().unwrap();
        dbg!(&query);
        assert!(query.contains_key("BTC/USD"));
    }
}
