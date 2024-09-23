use alpaca_api_client::{market_data::options::HistoricalOptionBarsQuery, TimeFrame};

fn main() {
    HistoricalOptionBarsQuery::new(
        vec!["AAPL241220C00300000", "AAPL241220P00300000"],
        TimeFrame::OneDay,
    )
    .start("2024-02-01")
    .end("2024-02-24")
    .limit(1)
    .sort_desc()
    .send()
    .unwrap();
}
