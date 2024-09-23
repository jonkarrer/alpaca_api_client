use alpaca_api_client::{
    market_data::stocks::{HistoricalBarsQuery, LatestBarsQuery},
    TimeFrame,
};

fn main() {
    historical_bar_query();
    latest_bar_query();
}

fn historical_bar_query() {
    HistoricalBarsQuery::new(vec!["AAPL"], TimeFrame::OneDay)
        .start("2022-02-01")
        .end("2022-02-10")
        .feed("iex")
        .send()
        .unwrap();
}

fn latest_bar_query() {
    LatestBarsQuery::new(vec!["AAPL", "TSLA"])
        .feed("iex")
        .send()
        .unwrap();
}
