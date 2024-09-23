use alpaca_api_client::trading::{
    account::{get_account, get_account_configurations, PatchAccountConfigQuery},
    AccountType,
};

fn main() {
    get_paper_account();
    get_live_account();
    get_account_config();
    patch_account_config();
}

fn get_paper_account() {
    get_account(AccountType::Paper).unwrap();
}

fn get_live_account() {
    get_account(AccountType::Live).unwrap();
}

fn get_account_config() {
    get_account_configurations(AccountType::Paper).unwrap();
}

fn patch_account_config() {
    PatchAccountConfigQuery::new()
        .ptp_no_exception_entry(false)
        .suspend_trade(false)
        .no_shorting(false)
        .pdt_check("entry")
        .send(AccountType::Paper)
        .unwrap();
}
