use alpaca_api_client::{market_data::stocks::bars::HistoricalBarsQuery, TimeFrame};

fn main() {
    let query = HistoricalBarsQuery::new(vec!["AAPL", "TSLA"], TimeFrame::OneDay)
        .start("2022-02-01")
        .end("2022-02-02")
        .feed("iex")
        .send()
        .unwrap();
    dbg!(&query);
}
