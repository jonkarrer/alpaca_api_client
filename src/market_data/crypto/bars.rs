use crate::{
    market_data::stocks::{HistoricalBars, HistoricalBarsResponse, LatestBars, LatestBarsResponse},
    request, TimeFrame,
};
use std::collections::HashMap;

pub struct HistoricalCryptoBarsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    timeframe: TimeFrame,
    start: Option<&'a str>,
    end: Option<&'a str>,
    limit: Option<i32>,
    sort_asc: bool,
    sort_desc: bool,
}

pub struct LatestCryptoBarsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
}

impl<'a> HistoricalCryptoBarsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>, timeframe: TimeFrame) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/bars",
            symbols,
            timeframe,
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

    pub fn sort_desc(mut self) -> Self {
        self.sort_desc = true;
        self.sort_asc = false;
        self
    }

    pub fn sort_asc(mut self) -> Self {
        self.sort_asc = true;
        self.sort_desc = false;
        self
    }

    fn build(&self) -> String {
        let mut query = format!(
            "symbols={}&timeframe={}",
            self.symbols.join(","),
            self.timeframe.to_string()
        );

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

    pub fn send(self) -> Result<HistoricalBars, ureq::Error> {
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
            let response: HistoricalBarsResponse = response.into_json()?;

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

impl<'a> LatestCryptoBarsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/latest/bars",
            symbols,
        }
    }

    fn build(self) -> String {
        let query = format!("symbols={}", self.symbols.join(","));
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<LatestBars, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;

        let response: LatestBarsResponse = response.into_json()?;

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
    fn test_historical_crypto_bars_query() {
        let test_symbol = "BTC/USD";
        let res = HistoricalCryptoBarsQuery::new(vec![test_symbol], TimeFrame::OneDay)
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key(test_symbol));
    }

    #[test]
    fn test_latest_crypto_bars_query() {
        let test_symbol = "BTC/USD";
        let res = LatestCryptoBarsQuery::new(vec![test_symbol])
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key(test_symbol));
    }
}
