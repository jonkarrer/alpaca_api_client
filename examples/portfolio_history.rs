use alpaca_api_client::trading::{portfolio::PortfolioHistoryQuery, AccountType};

fn main() {
    PortfolioHistoryQuery::new(AccountType::Paper)
        .start("2024-02-01")
        .end("2024-05-02")
        .pnl_reset("per_day")
        .send()
        .unwrap();
}
