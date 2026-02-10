use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct OptionTrade {
    pub t: String, // Timestamp
    pub x: String, // Exchange
    pub p: f64,    // Price
    pub s: i32,    // Size
    pub c: String, // Condition
}

pub type HistoricalOptionTrades = HashMap<String, Vec<OptionTrade>>;
pub type LatestOptionTrades = HashMap<String, OptionTrade>;

#[derive(Deserialize, Debug)]
pub struct HistoricalOptionTradesResponse {
    trades: HistoricalOptionTrades,
    next_page_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LatestOptionTradesResponse {
    trades: LatestOptionTrades,
}

pub struct HistoricalOptionTradesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    limit: Option<i32>,
    sort_asc: bool,
    sort_desc: bool,
}

pub struct LatestOptionTradesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
}

impl<'a> HistoricalOptionTradesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/trades",
            symbols,
            start: None,
            end: None,
            limit: None,
            sort_asc: false,
            sort_desc: false,
        }
    }

    pub fn start(mut self, start: &'a str) -> Self {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: &'a str) -> Self {
        self.end = Some(end);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn sort_asc(mut self) -> Self {
        self.sort_asc = true;
        self.sort_desc = false;
        self
    }

    pub fn sort_desc(mut self) -> Self {
        self.sort_desc = true;
        self.sort_asc = false;
        self
    }

    fn build(&self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));

        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }

        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }

        if self.sort_asc {
            query.push_str("&sort=asc");
        } else if self.sort_desc {
            query.push_str("&sort=desc");
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<HistoricalOptionTrades, ureq::Error> {
        let route = self.build();
        let mut trades: HistoricalOptionTrades = HashMap::new();
        let mut page_token = None;

        let mut i = 0;
        let data_limit = if let Some(limit) = self.limit {
            limit
        } else {
            1000
        };
        loop {
            if i >= data_limit {
                break;
            }

            // If a token exists, append to address
            let temp_address = match page_token {
                Some(token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };
            let response = request("GET", &temp_address).call()?;
            let response: HistoricalOptionTradesResponse = response.into_body().read_json()?;

            // Add trades to collection
            for (symbol, trade) in response.trades {
                i += trade.len() as i32;
                trades.entry(symbol).or_insert(Vec::new()).extend(trade);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(trades)
    }
}

impl<'a> LatestOptionTradesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/trades/latest",
            symbols,
            feed: None,
        }
    }

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    fn build(self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));

        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={}", feed));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<LatestOptionTrades, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: LatestOptionTradesResponse = response.into_body().read_json()?;
        Ok(response.trades)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_option_trade_query() {
        let res = HistoricalOptionTradesQuery::new(vec!["AAPL261218C00200000"])
            .limit(10)
            .send()
            .unwrap();
        dbg!(&res);
        assert!(res.contains_key("AAPL261218C00200000"));
    }

    #[test]
    fn test_latest_option_trade_query() {
        let res = LatestOptionTradesQuery::new(vec!["AAPL261218C00200000"])
            .feed("indicative")
            .send()
            .unwrap();
        dbg!(&res);
        assert!(res.contains_key("AAPL261218C00200000"));
    }
}
