use super::snapshot::{OptionSnapshotResponse, OptionSnapshots};
use crate::request;
use std::collections::HashMap;

pub struct OptionChainQuery<'a> {
    url: &'a str,
    underlying_symbol: &'a str,
    feed: Option<&'a str>,
    limit: Option<i32>,
    updated_since: Option<&'a str>,
    r#type: Option<&'a str>,
    strike_price_gte: Option<f64>,
    strike_price_lte: Option<f64>,
    expiration_date: Option<&'a str>,
    expiration_date_gte: Option<&'a str>,
    expiration_date_lte: Option<&'a str>,
    root_symbol: Option<&'a str>,
    sort_asc: bool,
    sort_desc: bool,
}

impl<'a> OptionChainQuery<'a> {
    pub fn new(underlying_symbol: &'a str) -> Self {
        Self {
            url: "https://data.alpaca.markets/v1beta1/options/snapshots",
            underlying_symbol,
            feed: None,
            limit: None,
            updated_since: None,
            r#type: None,
            strike_price_gte: None,
            strike_price_lte: None,
            expiration_date: None,
            expiration_date_gte: None,
            expiration_date_lte: None,
            root_symbol: None,
            sort_asc: false,
            sort_desc: false,
        }
    }

    pub fn feed(mut self, feed: &'a str) -> Self {
        self.feed = Some(feed);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn updated_since(mut self, updated_since: &'a str) -> Self {
        self.updated_since = Some(updated_since);
        self
    }

    pub fn set_type(mut self, r#type: &'a str) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn strike_price_gte(mut self, strike_price_gte: f64) -> Self {
        self.strike_price_gte = Some(strike_price_gte);
        self
    }

    pub fn strike_price_lte(mut self, strike_price_lte: f64) -> Self {
        self.strike_price_lte = Some(strike_price_lte);
        self
    }

    pub fn expiration_date(mut self, expiration_date: &'a str) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }

    pub fn expiration_date_gte(mut self, expiration_date_gte: &'a str) -> Self {
        self.expiration_date_gte = Some(expiration_date_gte);
        self
    }

    pub fn expiration_date_lte(mut self, expiration_date_lte: &'a str) -> Self {
        self.expiration_date_lte = Some(expiration_date_lte);
        self
    }

    pub fn root_symbol(mut self, root_symbol: &'a str) -> Self {
        self.root_symbol = Some(root_symbol);
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
        let mut query = format!("/{}?", self.underlying_symbol);

        if let Some(feed) = self.feed {
            query.push_str(&format!("&feed={}", feed));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }

        if let Some(updated_since) = self.updated_since {
            query.push_str(&format!("&updated_since={}", updated_since));
        }

        if let Some(r#type) = self.r#type {
            query.push_str(&format!("&type={}", r#type));
        }

        if let Some(strike_price_gte) = self.strike_price_gte {
            query.push_str(&format!("&strike_price_gte={}", strike_price_gte));
        }

        if let Some(strike_price_lte) = self.strike_price_lte {
            query.push_str(&format!("&strike_price_lte={}", strike_price_lte));
        }

        if let Some(expiration_date) = self.expiration_date {
            query.push_str(&format!("&expiration_date={}", expiration_date));
        }

        if let Some(expiration_date_gte) = self.expiration_date_gte {
            query.push_str(&format!("&expiration_date_gte={}", expiration_date_gte));
        }

        if let Some(expiration_date_lte) = self.expiration_date_lte {
            query.push_str(&format!("&expiration_date_lte={}", expiration_date_lte));
        }

        if let Some(root_symbol) = self.root_symbol {
            query.push_str(&format!("&root_symbol={}", root_symbol));
        }

        if self.sort_asc {
            query.push_str("&sort=asc");
        } else if self.sort_desc {
            query.push_str("&sort=desc");
        }

        format!("{}{}", self.url, query)
    }

    pub fn send(&self) -> Result<OptionSnapshots, ureq::Error> {
        let route = self.build();
        let mut snapshots: OptionSnapshots = HashMap::new();
        let mut page_token = None;

        let mut i = 0;
        let data_limit = if let Some(limit) = self.limit {
            limit
        } else {
            100
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
            let response: OptionSnapshotResponse = response.into_body().read_json()?;

            // Add snapshots to collection
            for (symbol, snapshot) in response.snapshots {
                i += 1;
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
    fn test_option_chain_query() {
        let res = OptionChainQuery::new("AAPL")
            .feed("indicative")
            .limit(10)
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.len() < 11);
    }
}
