use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct StockAuction {
    pub d: String,
    pub o: Option<Vec<StockPrice>>, // Open
    pub c: Option<Vec<StockPrice>>, // Close
    pub v: Option<Vec<StockPrice>>, // Volume
}

#[derive(Deserialize, Debug)]
pub struct StockPrice {
    pub c: String, // Condition
    pub p: f32,    // Price
    pub t: String, // Time
    pub s: i32,    // Size
    pub x: String, // Exchange
}

type HistoricalAuctions = HashMap<String, Vec<StockAuction>>;

#[derive(Deserialize, Debug)]
pub struct HistoricalAuctionResponse {
    pub auctions: HistoricalAuctions,
    pub next_page_token: Option<String>,
}

pub struct HistoricalAuctionsQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    feed: Option<&'a str>,
    currency: Option<&'a str>,
    limit: Option<i32>,
    asof: Option<&'a str>,
    sort_asc: bool,
    sort_desc: bool,
}

impl<'a> HistoricalAuctionsQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/auctions",
            symbols,
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

    fn build(self) -> String {
        let mut query = format!("symbols={}", self.symbols.join(","));

        if let Some(start) = self.start {
            query.push_str(&format!("&start={}", start));
        }

        if let Some(end) = self.end {
            query.push_str(&format!("&end={}", end));
        }

        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={}", feed));
        }

        if let Some(currency) = self.currency {
            query.push_str(&format!("&currency={}", currency));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }

        if let Some(asof) = self.asof {
            query.push_str(&format!("&asof={}", asof));
        }

        if self.sort_asc {
            query.push_str("&sort=asc");
        } else if self.sort_desc {
            query.push_str("&sort=desc");
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<HistoricalAuctions, ureq::Error> {
        let route = self.build();
        let mut auctions: HistoricalAuctions = HashMap::new();
        let mut page_token = None;

        loop {
            // If a token exists, append to address
            let temp_address = match page_token {
                Some(token) => format!("{}&page_token={}", &route, &token),
                _ => route.clone(),
            };
            let response = request("GET", &temp_address).call()?;
            let response: HistoricalAuctionResponse = response.into_body().read_json()?;

            // Add auctions to collection
            for (symbol, auction) in response.auctions {
                auctions.entry(symbol).or_insert(Vec::new()).extend(auction);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(auctions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_auction_query() {
        let res = HistoricalAuctionsQuery::new(vec!["AAPL"])
            .feed("sip")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL"));
    }
}
