use super::request;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub account_blocked: bool,
    pub account_number: String,
    pub crypto_status: String,
    pub buying_power: String,
    pub cash: String,
    pub created_at: String,
    pub currency: String,
    pub non_marginable_buying_power: String,
    pub accrued_fees: String,
    pub pending_transfer_in: Option<String>,
    pub pending_transfer_out: Option<String>,
    pub daytrade_count: u64,
    pub daytrading_buying_power: String,
    pub equity: String,
    pub id: String,
    pub initial_margin: String,
    pub last_equity: String,
    pub last_maintenance_margin: String,
    pub long_market_value: String,
    pub maintenance_margin: String,
    pub multiplier: String,
    pub pattern_day_trader: bool,
    pub portfolio_value: String,
    pub regt_buying_power: String,
    pub short_market_value: String,
    pub shorting_enabled: bool,
    pub sma: String,
    pub status: String,
    pub trade_suspended_by_user: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
}

pub fn get_account() -> Account {
    let address = "https://paper-api.alpaca.markets/v2/account";

    let r: Account = request("GET", address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");
    r
}
