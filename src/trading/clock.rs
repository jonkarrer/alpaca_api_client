use crate::request;

use super::AccountType;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MarketClock {
    pub timestamp: String,
    pub is_open: bool,
    pub next_open: String,
    pub next_close: String,
}

pub fn get_market_clock(account_type: AccountType) -> Result<MarketClock, ureq::Error> {
    let url = match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/clock",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/clock",
    };

    let response = request("GET", url).call()?;
    Ok(response.into_body().read_json()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_market_clock() {
        let res = get_market_clock(AccountType::Paper).unwrap();
        dbg!(&res);
        assert!(res.timestamp.len() > 0);
    }
}
