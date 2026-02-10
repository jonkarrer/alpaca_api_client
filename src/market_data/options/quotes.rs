use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct OptionQuote {
    pub t: String,  // Timestamp
    pub ax: String, // Exchange
    pub ap: f32,    // Ask Price
    pub r#as: i32,  // Ask Size
    pub bx: String, // Exchange
    pub bp: f32,    // Bid Price
    pub bs: i32,    // Bid Size
    pub c: String,  // Condition
}

pub type LatestOptionQuotes = HashMap<String, OptionQuote>;

#[derive(Deserialize, Debug)]
pub struct LatestOptionQuotesResponse {
    quotes: LatestOptionQuotes,
}

pub struct LatestOptionQuotesQuery<'a> {
    url: &'a str,
    symbols: Vec<&'a str>,
    feed: Option<&'a str>,
}

impl<'a> LatestOptionQuotesQuery<'a> {
    pub fn new(symbols: Vec<&'a str>) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/quotes/latest",
            symbols,
            feed: None,
        }
    }

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    fn build(&self) -> String {
        let mut url = format!("{}?symbols={}", self.url, self.symbols.join(","));
        if let Some(feed) = self.feed {
            url = format!("{}&feed={}", url, feed);
        }
        url
    }

    pub fn send(self) -> Result<LatestOptionQuotes, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let response: LatestOptionQuotesResponse = response.into_body().read_json()?;
        Ok(response.quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latest_option_quotes_query() {
        let res = LatestOptionQuotesQuery::new(vec!["AAPL241220C00300000"])
            .feed("indicative")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.contains_key("AAPL241220C00300000"));
    }
}
