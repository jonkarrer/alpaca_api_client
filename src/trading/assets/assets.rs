use crate::{request, trading::AccountType};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: String,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
    pub margin_requirement_long: Option<String>,
    pub margin_requirement_short: Option<String>,
    pub attributes: Option<Vec<String>>,
}

pub type AllAssets = Vec<Asset>;

pub struct AssetsQuery<'a> {
    pub url: &'a str,
    pub status: Option<&'a str>,
    pub asset_class: Option<&'a str>,
    pub exchange: Option<&'a str>,
    pub attributes: Option<Vec<&'a str>>,
}

impl<'a> AssetsQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        Self {
            url: match account_type {
                AccountType::Live => "https://api.alpaca.markets/v2/assets",
                AccountType::Paper => "https://paper-api.alpaca.markets/v2/assets",
            },
            status: None,
            asset_class: None,
            exchange: None,
            attributes: None,
        }
    }

    pub fn get_by_id(self, id: &'a str) -> Result<Asset, ureq::Error> {
        let route = format!("{}/{}", self.url, id);
        let response = request("GET", &route).call()?;
        let asset: Asset = response.into_json()?;
        Ok(asset)
    }

    pub fn get_by_symbol(self, symbol: &'a str) -> Result<Asset, ureq::Error> {
        let route = format!("{}/{}", self.url, symbol);
        let response = request("GET", &route).call()?;
        let asset: Asset = response.into_json()?;
        Ok(asset)
    }

    pub fn status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    pub fn asset_class(mut self, asset_class: &'a str) -> Self {
        self.asset_class = Some(asset_class);
        self
    }

    pub fn exchange(mut self, exchange: &'a str) -> Self {
        self.exchange = Some(exchange);
        self
    }

    pub fn attributes(mut self, attributes: Vec<&'a str>) -> Self {
        self.attributes = Some(attributes);
        self
    }

    pub fn build(self) -> String {
        let mut query = String::new();
        if let Some(status) = self.status {
            query.push_str(&format!("&status={}", status));
        }
        if let Some(asset_class) = self.asset_class {
            query.push_str(&format!("&asset_class={}", asset_class));
        }
        if let Some(exchange) = self.exchange {
            query.push_str(&format!("&exchange={}", exchange));
        }
        if let Some(attributes) = self.attributes {
            query.push_str(&format!("&attributes={}", attributes.join(",")));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(self) -> Result<AllAssets, ureq::Error> {
        let route = self.build();
        let response = request("GET", &route).call()?;
        let assets: AllAssets = response.into_json()?;
        Ok(assets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_assets_query() {
        let res = AssetsQuery::new(AccountType::Paper)
            .status("active")
            .send()
            .unwrap();

        dbg!(&res);
        assert!(res.len() > 0);
    }

    #[test]
    fn test_get_asset_by_id() {
        let res = AssetsQuery::new(AccountType::Paper)
            .get_by_id("AAPL")
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "AAPL");
    }

    #[test]
    fn test_get_asset_by_symbol() {
        let res = AssetsQuery::new(AccountType::Paper)
            .get_by_symbol("AAPL")
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "AAPL");
    }
}
