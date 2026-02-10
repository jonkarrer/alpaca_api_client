use crate::{json_request, request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::AccountType;

#[derive(Deserialize, Debug)]
pub struct AccountConfiguration {
    pub dtbp_check: Option<String>,
    pub trade_confirm_email: Option<String>,
    pub suspend_trade: Option<bool>,
    pub no_shorting: Option<bool>,
    pub fractional_trading: Option<bool>,
    pub max_margin_multiplier: Option<String>,
    pub max_options_trading_level: Option<u64>,
    pub pdt_check: Option<String>,
    pub ptp_no_exception_entry: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub admin_configurations: HashMap<String, String>,
    #[serde(deserialize_with = "crate::serde::deserialize_to_string_map")]
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

pub fn get_account(account_type: AccountType) -> Result<Account, ureq::Error> {
    let url = match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/account",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/account",
    };
    let response = request("GET", &url).call()?;
    Ok(response.into_body().read_json()?)
}

pub fn get_account_configurations(
    account_type: AccountType,
) -> Result<AccountConfiguration, ureq::Error> {
    let url = match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/account/configurations",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/account/configurations",
    };
    let response = request("GET", &url).call()?;
    Ok(response.into_body().read_json()?)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PatchAccountConfigQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    dtbp_check: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    trade_confirm_email: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    suspend_trade: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    no_shorting: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    fractional_trading: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_margin_multiplier: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_options_trading_level: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pdt_check: Option<&'a str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ptp_no_exception_entry: Option<bool>,
}

impl<'a> PatchAccountConfigQuery<'a> {
    pub fn new() -> Self {
        Self {
            dtbp_check: None,
            trade_confirm_email: None,
            suspend_trade: None,
            no_shorting: None,
            fractional_trading: None,
            max_margin_multiplier: None,
            max_options_trading_level: None,
            pdt_check: None,
            ptp_no_exception_entry: None,
        }
    }

    pub fn dtbp_check(mut self, dtbp_check: &'a str) -> Self {
        self.dtbp_check = Some(dtbp_check);
        self
    }

    pub fn trade_confirm_email(mut self, trade_confirm_email: &'a str) -> Self {
        self.trade_confirm_email = Some(trade_confirm_email);
        self
    }

    pub fn suspend_trade(mut self, suspend_trade: bool) -> Self {
        self.suspend_trade = Some(suspend_trade);
        self
    }

    pub fn no_shorting(mut self, no_shorting: bool) -> Self {
        self.no_shorting = Some(no_shorting);
        self
    }

    pub fn fractional_trading(mut self, fractional_trading: bool) -> Self {
        self.fractional_trading = Some(fractional_trading);
        self
    }

    pub fn max_margin_multiplier(mut self, max_margin_multiplier: &'a str) -> Self {
        self.max_margin_multiplier = Some(max_margin_multiplier);
        self
    }

    pub fn max_options_trading_level(mut self, max_options_trading_level: u64) -> Self {
        self.max_options_trading_level = Some(max_options_trading_level);
        self
    }

    pub fn pdt_check(mut self, pdt_check: &'a str) -> Self {
        self.pdt_check = Some(pdt_check);
        self
    }

    pub fn ptp_no_exception_entry(mut self, ptp_no_exception_entry: bool) -> Self {
        self.ptp_no_exception_entry = Some(ptp_no_exception_entry);
        self
    }

    pub fn send(self, account_type: AccountType) -> Result<AccountConfiguration, ureq::Error> {
        let url = match account_type {
            AccountType::Live => "https://api.alpaca.markets/v2/account/configurations",
            AccountType::Paper => "https://paper-api.alpaca.markets/v2/account/configurations",
        };

        let response = json_request("PATCH", url)
            .header("Content-Type", "application/json")
            .send_json(&self)?;

        Ok(response.into_body().read_json()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paper_account() {
        let account = get_account(AccountType::Paper).unwrap();
        dbg!(&account);
        assert!(account.status == "ACTIVE");
    }

    #[test]
    fn test_get_account_config() {
        let account = get_account_configurations(AccountType::Paper).unwrap();
        dbg!(&account);
        assert!(account.suspend_trade.unwrap() == false);
    }

    #[test]
    fn test_patch_account_config() {
        let account = PatchAccountConfigQuery::new()
            .ptp_no_exception_entry(false)
            .send(AccountType::Paper)
            .unwrap();

        dbg!(&account);
        assert!(account.ptp_no_exception_entry.unwrap() == false);
    }
}
