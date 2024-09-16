use crate::request;
use serde::Deserialize;
use std::collections::HashMap;

/// API object for an Account
#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub admin_configurations: HashMap<String, String>,
    pub user_configurations: Option<HashMap<String, String>>,
    pub account_number: String,
    pub status: String,
    pub crypto_status: String,
    pub currency: String,
    pub buying_power: String,
    pub regt_buying_power: String,
    pub daytrading_buying_power: String,
    pub options_buying_power: String,
    pub effective_buying_power: String,
    pub non_marginable_buying_power: String,
    pub bod_dtbp: String,
    pub cash: String,
    pub accrued_fees: String,
    pub pending_transfer_in: Option<String>,
    pub portfolio_value: String,
    pub pattern_day_trader: bool,
    pub trading_blocked: bool,
    pub transfers_blocked: bool,
    pub account_blocked: bool,
    pub created_at: String,
    pub trade_suspended_by_user: bool,
    pub multiplier: String,
    pub shorting_enabled: bool,
    pub equity: String,
    pub last_equity: String,
    pub long_market_value: String,
    pub short_market_value: String,
    pub initial_margin: String,
    pub maintenance_margin: String,
    pub last_maintenance_margin: String,
    pub sma: String,
    pub daytrade_count: i32,
    pub balance_asof: String,
    pub crypto_tier: usize,
    pub options_trading_level: usize,
    pub intraday_adjustments: String,
    pub pending_reg_taf_fees: String,
}

impl Account {
    /// Get your paper account details
    pub fn get_paper_account() -> Result<Account, ureq::Error> {
        let address = "https://paper-api.alpaca.markets/v2/account";

        let response = request("GET", &address).call()?;
        let account: Account = response.into_json()?;

        Ok(account)
    }

    /// Get your live account details
    pub fn get_live_account() -> Result<Account, ureq::Error> {
        let address = "https://api.alpaca.markets/v2/account";

        let response = request("GET", &address).call()?;
        let account: Account = response.into_json()?;

        Ok(account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paper_account() {
        let account = Account::get_paper_account().unwrap();
        dbg!(&account);
        assert!(false);
    }

    #[test]
    fn test_get_live_account() {
        let account = Account::get_live_account().unwrap();
        dbg!(&account);
        assert!(false);
    }
}
