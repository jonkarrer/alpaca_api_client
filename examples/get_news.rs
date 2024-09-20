use alpaca_api_client::market_data::news::NewsQuery;

fn main() {
    let query = NewsQuery::new(vec!["AAPL"])
        .include_content(true)
        .exclude_contentless(true)
        .limit(2)
        .sort_desc()
        .send()
        .unwrap();

    dbg!(&query);
}
