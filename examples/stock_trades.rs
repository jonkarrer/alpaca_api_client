use alpaca_api_client::market_data::stocks::{HistoricalTradesQuery, LatestTradesQuery};

fn main() {
    historical_trade_query();
    latest_trade_query();
}

fn historical_trade_query() {
    HistoricalTradesQuery::new(vec!["AAPL", "TSLA", "PLTR"])
        .feed("sip")
        .sort_desc()
        .currency("USD")
        .limit(10)
        .send()
        .unwrap();
}

fn latest_trade_query() {
    LatestTradesQuery::new(vec!["AAPL", "TSLA"])
        .feed("iex")
        .send()
        .unwrap();
}
