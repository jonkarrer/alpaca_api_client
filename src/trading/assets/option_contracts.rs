use serde::Deserialize;

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
    pub type_: String,
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

pub struct OptionContractsResponse {
    pub option_contracts: AllOptionContracts,
    pub next_page_token: Option<String>,
}

pub struct OptionContractsQuery<'a> {
    pub url: &'a str,
    pub underlying_symbols: Option<&'a str>,
    pub show_deliverables: Option<bool>,
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
