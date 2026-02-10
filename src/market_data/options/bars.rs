use crate::{
    market_data::stocks::{HistoricalBars, HistoricalBarsResponse},
    request, TimeFrame,
};
use std::collections::HashMap;

pub struct HistoricalOptionBarsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    timeframe: TimeFrame,
    start: Option<&'a str>,
    end: Option<&'a str>,
    limit: Option<i32>,
    sort_asc: bool,
    sort_desc: bool,
}

impl<'a> HistoricalOptionBarsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>, timeframe: TimeFrame) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/bars",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_option_bars_query() {
        let test_symbol = "AAPL261218C00200000";
        let res = HistoricalOptionBarsQuery::new(vec![test_symbol], TimeFrame::OneDay)
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key(test_symbol));
    }
}
