use alpaca_api_client::{
    market_data::crypto::bars::{HistoricalCryptoBarsQuery, LatestCryptoBarsQuery},
    TimeFrame,
};

fn main() {
    historical_crypto_bars_query();
    latest_crypto_bars_query();
}

fn historical_crypto_bars_query() {
    let test_symbol = "BTC/USD";
    HistoricalCryptoBarsQuery::new(vec![test_symbol], TimeFrame::OneDay)
        .sort_asc()
        .limit(10)
        .send()
        .unwrap();
}

fn latest_crypto_bars_query() {
    let test_symbol = "BTC/USD";
    LatestCryptoBarsQuery::new(vec![test_symbol])
        .send()
        .unwrap();
}
