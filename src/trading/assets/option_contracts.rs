use serde::Deserialize;

use crate::{request, trading::AccountType};

#[derive(Deserialize, Debug)]
pub struct OptionContract {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub status: String,
    pub tradable: bool,
    pub expiration_date: String,
    pub root_symbol: String,
    pub underlying_symbol: String,
    pub underlying_asset_id: String,
    pub r#type: String,
    pub style: String,
    pub strike_price: String,
    pub multiplier: String,
    pub size: String,
    pub open_interest: Option<String>,
    pub open_interest_date: Option<String>,
    pub close_price: Option<String>,
    pub close_price_date: Option<String>,
    pub deliverables: Option<Vec<Deliverable>>,
}

#[derive(Deserialize, Debug)]
pub struct Deliverable {
    pub r#type: String,
    pub symbol: String,
    pub asset_id: String,
    pub amount: String,
    pub allocation_percentage: String,
    pub settlement_type: String,
    pub settlement_method: String,
    pub delayed_settlement: bool,
}

pub type AllOptionContracts = Vec<OptionContract>;

#[derive(Deserialize, Debug)]
pub struct OptionContractsResponse {
    pub option_contracts: AllOptionContracts,
    pub next_page_token: Option<String>,
}

pub struct OptionContractsQuery<'a> {
    pub url: &'a str,
    pub underlying_symbols: Option<Vec<&'a str>>,
    pub show_deliverables: bool,
    pub status: Option<&'a str>,
    pub expiration_date: Option<&'a str>,
    pub expiration_date_gte: Option<&'a str>,
    pub expiration_date_lte: Option<&'a str>,
    pub root_symbol: Option<&'a str>,
    pub r#type: Option<&'a str>,
    pub style: Option<&'a str>,
    pub strike_price_gte: Option<&'a str>,
    pub strike_price_lte: Option<&'a str>,
    pub limit: Option<usize>,
    pub ppind: Option<bool>,
}

impl<'a> OptionContractsQuery<'a> {
    pub fn new(account_type: AccountType) -> Self {
        let url = match account_type {
            AccountType::Live => "https://api.alpaca.markets/v2/options/contracts",
            AccountType::Paper => "https://paper-api.alpaca.markets/v2/options/contracts",
        };

        Self {
            url,
            underlying_symbols: None,
            show_deliverables: false,
            status: None,
            expiration_date: None,
            expiration_date_gte: None,
            expiration_date_lte: None,
            root_symbol: None,
            r#type: None,
            style: None,
            strike_price_gte: None,
            strike_price_lte: None,
            limit: None,
            ppind: None,
        }
    }

    pub fn underlying_symbols(mut self, underlying_symbols: Vec<&'a str>) -> Self {
        self.underlying_symbols = Some(underlying_symbols);
        self
    }

    pub fn show_deliverables(mut self, show_deliverables: bool) -> Self {
        self.show_deliverables = show_deliverables;
        self
    }

    pub fn status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
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

    pub fn set_type(mut self, r#type: &'a str) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn style(mut self, style: &'a str) -> Self {
        self.style = Some(style);
        self
    }

    pub fn strike_price_gte(mut self, strike_price_gte: &'a str) -> Self {
        self.strike_price_gte = Some(strike_price_gte);
        self
    }

    pub fn strike_price_lte(mut self, strike_price_lte: &'a str) -> Self {
        self.strike_price_lte = Some(strike_price_lte);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn ppind(mut self, ppind: bool) -> Self {
        self.ppind = Some(ppind);
        self
    }

    fn build(&self) -> String {
        let mut query = String::new();
        if let Some(underlying_symbols) = &self.underlying_symbols {
            query.push_str(&format!(
                "&underlying_symbols={}",
                underlying_symbols.join(",")
            ));
        }
        if self.show_deliverables {
            query.push_str("&show_deliverables=true");
        }
        if let Some(status) = self.status {
            query.push_str(&format!("&status={}", status));
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
        if let Some(r#type) = self.r#type {
            query.push_str(&format!("&type={}", r#type));
        }
        if let Some(style) = self.style {
            query.push_str(&format!("&style={}", style));
        }
        if let Some(strike_price_gte) = self.strike_price_gte {
            query.push_str(&format!("&strike_price_gte={}", strike_price_gte));
        }
        if let Some(strike_price_lte) = self.strike_price_lte {
            query.push_str(&format!("&strike_price_lte={}", strike_price_lte));
        }
        if let Some(limit) = self.limit {
            query.push_str(&format!("&limit={}", limit));
        }
        if let Some(ppind) = self.ppind {
            query.push_str(&format!("&ppind={}", ppind));
        }

        format!("{}?{}", self.url, query)
    }

    pub fn send(&self) -> Result<AllOptionContracts, ureq::Error> {
        let route = self.build();
        let mut option_contracts: AllOptionContracts = Vec::new();
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
            let response: OptionContractsResponse = response.into_json()?;

            // Add option_contracts to collection
            for contract in response.option_contracts {
                i += 1;
                option_contracts.push(contract);
            }

            // If a token is in response, assign to page_token for next loop
            match response.next_page_token {
                Some(next_page_token) => page_token = Some(next_page_token.clone()),
                _ => break,
            }
        }

        Ok(option_contracts)
    }

    pub fn get_by_id(self, id: &'a str) -> Result<OptionContract, ureq::Error> {
        let route = format!("{}/{}", self.url, id);
        let response = request("GET", &route).call()?;
        let asset: OptionContract = response.into_json()?;
        Ok(asset)
    }

    pub fn get_by_symbol(self, symbol: &'a str) -> Result<OptionContract, ureq::Error> {
        let route = format!("{}/{}", self.url, symbol);
        let response = request("GET", &route).call()?;
        let asset: OptionContract = response.into_json()?;
        Ok(asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_option_contracts_query() {
        let query = OptionContractsQuery::new(AccountType::Paper)
            .limit(10)
            .send()
            .unwrap();

        dbg!(&query);
        assert!(query.len() < 11);
    }

    #[test]
    fn test_get_option_contract_by_id() {
        let res = OptionContractsQuery::new(AccountType::Paper)
            .get_by_id("79fd0e4b-255c-4016-91f8-4c5304a16c76")
            .unwrap();
        dbg!(&res);
        assert!(res.id == "79fd0e4b-255c-4016-91f8-4c5304a16c76");
    }

    #[test]
    fn test_get_option_contract_by_symbol() {
        let res = OptionContractsQuery::new(AccountType::Paper)
            .get_by_symbol("A240920C00105000")
            .unwrap();
        dbg!(&res);
        assert!(res.symbol == "A240920C00105000");
    }
}
