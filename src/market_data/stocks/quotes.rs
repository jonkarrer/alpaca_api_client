use serde::Deserialize;
use std::collections::HashMap;

use crate::request;

#[derive(Deserialize, Debug)]
pub struct StockQuote {
    t: String,      // Timestamp
    ax: String,     // Exchange
    ap: f32,        // Ask Price
    r#as: i32,      // Ask Size
    bx: String,     // Exchange
    bp: f32,        // Bid Price
    bs: i32,        // Bid Size
    c: Vec<String>, // Condition
    z: String,      // Condition
}

type HistoricalQuotes = HashMap<String, Vec<StockQuote>>;
type LatestQuotes = HashMap<String, StockQuote>;

#[derive(Deserialize, Debug)]
pub struct HistoricalQuotesResponse {
    quotes: HistoricalQuotes,
    next_page_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LatestQuotesResponse {
    quotes: HashMap<String, StockQuote>,
}

pub struct HistoricalQuotesQuery<'a> {
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

pub struct LatestQuotesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
    currency: Option<&'a str>,
}

impl<'a> HistoricalQuotesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/quotes",
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

    fn build(&self) -> String {
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

    pub fn send(&self) -> Result<HistoricalQuotes, ureq::Error> {
        let route = self.build();
        let mut quotes: HistoricalQuotes = HashMap::new();
        let mut page_token: Option<String> = None;

        // this endpoint returns page tokens no matter what.so we need to apply the limit. Default is 1000.
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
            let response: HistoricalQuotesResponse = response.into_json()?;

            // Add quotes to collection
            for (symbol, quote) in response.quotes {
                i += quote.len() as i32;
                quotes.entry(symbol).or_insert(Vec::new()).extend(quote);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        dbg!(i);

        Ok(quotes)
    }
}

impl<'a> LatestQuotesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v2/stocks/quotes/latest",
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

    pub fn send(self) -> Result<LatestQuotes, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;

        let response: LatestQuotesResponse = response.into_json()?;

        let mut latest_quotes: LatestQuotes = HashMap::new();

        for (symbol, quote) in response.quotes {
            latest_quotes.insert(symbol, quote);
        }

        Ok(latest_quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_quotes_query() {
        let res = HistoricalQuotesQuery::new(vec!["AAPL"])
            .feed("sip")
            .limit(5)
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL"));
    }

    #[test]
    fn test_latest_quotes_query() {
        let res = LatestQuotesQuery::new(vec!["AAPL"])
            .feed("iex")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL"));
    }
}
