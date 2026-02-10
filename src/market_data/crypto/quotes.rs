use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct CryptoQuote {
    pub t: String,
    pub bp: f64,
    pub bs: f64,
    pub ap: f64,
}

pub type HistoricalCryptoQuotes = HashMap<String, Vec<CryptoQuote>>;
pub type LatestCryptoQuotes = HashMap<String, CryptoQuote>;

#[derive(Deserialize, Debug)]
pub struct HistoricalCryptoQuoteResponse {
    quotes: HistoricalCryptoQuotes,
    next_page_token: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LatestCryptoQuoteResponse {
    pub quotes: LatestCryptoQuotes,
}

pub struct HistoricalCryptoQuotesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    start: Option<&'a str>,
    end: Option<&'a str>,
    limit: Option<i32>,
    sort_asc: bool,
    sort_desc: bool,
}

pub struct LatestCryptoQuotesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
}

impl<'a> HistoricalCryptoQuotesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/quotes",
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
            query = format!("{}&start={}", query, start);
        }

        if let Some(end) = self.end {
            query = format!("{}&end={}", query, end);
        }

        if let Some(limit) = self.limit {
            query = format!("{}&limit={}", query, limit);
        }

        if self.sort_asc {
            query = format!("{}&sort=asc", query);
        } else if self.sort_desc {
            query = format!("{}&sort=desc", query);
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<HistoricalCryptoQuotes, ureq::Error> {
        let route = self.build();
        let mut quotes: HistoricalCryptoQuotes = HashMap::new();
        let mut page_token: Option<String> = None;

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
            let response: HistoricalCryptoQuoteResponse = response.into_body().read_json()?;

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

        Ok(quotes)
    }
}

impl<'a> LatestCryptoQuotesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta3/crypto/us/latest/quotes",
            symbols,
        }
    }

    fn build(self) -> String {
        let query = format!("symbols={}", self.symbols.join(","));
        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<LatestCryptoQuotes, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: LatestCryptoQuoteResponse = response.into_body().read_json()?;
        Ok(response.quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historical_crypto_quotes_query() {
        let query = HistoricalCryptoQuotesQuery::new(vec!["BTC/USD"])
            .limit(10)
            .sort_desc()
            .send()
            .unwrap();

        dbg!(&query);
        assert!(query.contains_key("BTC/USD"));
    }

    #[test]
    fn test_latest_crypto_quotes_query() {
        let query = LatestCryptoQuotesQuery::new(vec!["BTC/USD"])
            .send()
            .unwrap();

        dbg!(&query);
        assert!(query.contains_key("BTC/USD"));
    }
}
