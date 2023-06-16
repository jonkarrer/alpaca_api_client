use super::request;
use serde::Deserialize;
use std::collections::HashMap;

// A Bar is a candle in stock market terms
#[derive(Deserialize, Debug)]
pub struct Bar {
    pub t: String, // Timestamp
    pub o: f32,    // Open
    pub h: f32,    // High
    pub l: f32,    // Low
    pub c: f32,    // Close
    pub v: i32,    // Volume
    pub n: i32,    // Number of trades
    pub vw: f32,   // Volume weighted average
}

pub type MultiBars = HashMap<String, Vec<Bar>>;

pub fn get_bars(stock_symbol: &str, timeframe: &str, query: Option<&str>) -> Vec<Bar> {
    let url =
        format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/bars?timeframe={timeframe}");
    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct Res {
        bars: Option<Vec<Bar>>,
        // symbol: String,
        next_page_token: Option<String>,
    }

    let mut bars = Vec::new();
    let mut page_token = None;

    loop {
        // If a token exists, append to address
        let temp_address = match page_token {
            Some(token) => format!("{}&page_token={}", &address, &token),
            _ => address.clone(),
        };
        let r: Res = request("GET", &temp_address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        // If a token is in response, assign to page_token for next loop
        match r.next_page_token {
            Some(next_page_token) => {
                page_token = Some(next_page_token.clone());
                // Collect results into Vec
                bars.extend(r.bars.unwrap());
            }
            _ => {
                bars.extend(r.bars.unwrap());
                break;
            }
        }
    }
    bars
}

pub fn get_multi_bars(stock_symbols: &[&str], timeframe: &str, query: Option<&str>) -> MultiBars {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/bars?timeframe={timeframe}&symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct Res {
        bars: Option<HashMap<String, Vec<Bar>>>,
        next_page_token: Option<String>,
    }

    let mut stock_bars_map: MultiBars = HashMap::new();
    let mut page_token = None;

    loop {
        // If a token exists, append to address
        let temp_address = match page_token {
            Some(token) => format!("{}&page_token={}", &address, &token),
            _ => address.clone(),
        };
        let r: Res = request("GET", &temp_address)
            .call()
            .expect("Could Not Call API")
            .into_json()
            .expect("Could Not Parse Response Into Json");

        // If a token is in response, assign to page_token for next loop
        match r.next_page_token {
            Some(next_page_token) => {
                page_token = Some(next_page_token.clone());
                // Collect results into HashMap
                for (symbol, bars) in r.bars.unwrap() {
                    stock_bars_map
                        .entry(symbol)
                        .or_insert(Vec::new())
                        .extend(bars);
                }
            }
            _ => {
                for (symbol, bars) in r.bars.unwrap() {
                    stock_bars_map
                        .entry(symbol)
                        .or_insert(Vec::new())
                        .extend(bars);
                }
                break;
            }
        }
    }
    stock_bars_map
}
