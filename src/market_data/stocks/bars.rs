use crate::{request, TimeFrame};
use serde::Deserialize;
use std::collections::HashMap;

/// API object for a Bar
#[derive(Deserialize, Debug)]
pub struct StockBar {
    pub t: String, // Timestamp
    pub o: f32,    // Open
    pub h: f32,    // High
    pub l: f32,    // Low
    pub c: f32,    // Close
    pub v: f32,    // Volume
    pub n: i32,    // Number of trades
    pub vw: f32,   // Volume weighted average
}

/// A custom type for a hashmap of stock symbols and bars
pub type HistoricalBars = HashMap<String, Vec<StockBar>>;

/// A custom type for a hashmap of stock symbols and latest bars
pub type LatestBars = HashMap<String, StockBar>;

#[derive(Deserialize)]
pub struct HistoricalBarsResponse {
    pub bars: HistoricalBars,
    pub next_page_token: Option<String>,
}

#[derive(Deserialize)]
pub struct LatestBarsResponse {
    pub bars: LatestBars,
}

pub struct HistoricalBarsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    timeframe: TimeFrame,
    start: Option<&'a str>,
    end: Option<&'a str>,
    feed: Option<&'a str>,
    currency: Option<&'a str>,
    limit: Option<i32>,
    asof: Option<&'a str>,
    sort_asc: bool,
    sort_desc: bool,
}

pub struct LatestBarsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
    currency: Option<&'a str>,
}

impl<'a> HistoricalBarsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>, timeframe: TimeFrame) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/bars",
            symbols,
            timeframe,
            start: None,
            end: None,
            feed: None,
            currency: None,
            limit: None,
            asof: None,
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

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    pub fn currency(mut self, currency: &'a str) -> Self {
        self.currency = Some(currency);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn asof(mut self, asof: &'a str) -> Self {
        self.asof = Some(asof);
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
        let mut query = format!(
            "symbols={}&timeframe={}",
            self.symbols.join(","),
            self.timeframe.to_string()
        );
        if let Some(start) = self.start {
            query.push_str(&format!("&start={start}"));
        }
        if let Some(end) = self.end {
            query.push_str(&format!("&end={end}"));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={limit}"));
        }
        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={feed}"));
        }
        if let Some(currency) = self.currency {
            query.push_str(&format!("&currency={currency}"));
        }
        if let Some(asof) = self.asof {
            query.push_str(&format!("&asof={asof}"));
        }
        if self.sort_asc {
            query.push_str("&sort=asc");
        }
        if self.sort_desc {
            query.push_str("&sort=desc");
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<HistoricalBars, ureq::Error> {
        let route = self.build();
        let mut multi_bars: HistoricalBars = HashMap::new();
        let mut page_token = None;

        loop {
            // If a token exists, append to address
            let temp_address = match page_token {
                Some(token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };
            let response = request("GET", &temp_address).call()?;
            let response: HistoricalBarsResponse = response.into_body().read_json()?;

            // Add multi_bars to collection
            for (symbol, bars) in response.bars {
                multi_bars.entry(symbol).or_insert(Vec::new()).extend(bars);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(multi_bars)
    }
}

impl<'a> LatestBarsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/bars/latest",
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
            query.push_str(&format!("&feed={feed}"));
        }
        if let Some(currency) = self.currency {
            query.push_str(&format!("&currency={currency}"));
        }
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<LatestBars, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;

        let response: LatestBarsResponse = response.into_body().read_json()?;

        let mut latest_bars: LatestBars = HashMap::new();

        for (symbol, bars) in response.bars {
            latest_bars.insert(symbol, bars);
        }

        Ok(latest_bars)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_historical_bar_query() {
        let res = HistoricalBarsQuery::new(vec!["AAPL"], TimeFrame::OneDay)
            .start("2022-02-01")
            .end("2022-03-10")
            .feed("iex")
            .limit(2)
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL"));
    }

    #[test]
    fn test_latest_bar_query() {
        let res = LatestBarsQuery::new(vec!["AAPL", "TSLA"])
            .feed("iex")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("TSLA"));
    }
}
