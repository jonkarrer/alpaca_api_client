use super::request;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub t: String,      // Timestamp
    pub x: String,      // Exchange
    pub p: f32,         // Price
    pub s: f32,         // Trade Size
    pub c: Vec<String>, // Trade Conditions
    pub i: i64,         // Trade ID
    pub z: String,      // Tape
}

pub fn get_trades(stock_symbol: &str, query: Option<&str>) -> Vec<Trade> {
    let url = format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/trades");
    let address = match query {
        Some(query) => format!("{url}?{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Res {
        trades: Option<Vec<Trade>>,
        symbol: String,
        next_page_token: Option<String>,
    }
    let mut trades = Vec::new();
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
                trades.extend(r.trades.unwrap());
            }
            _ => {
                trades.extend(r.trades.unwrap());
                break;
            }
        }
    }
    trades
}

pub type MultiTrades = HashMap<String, Vec<Trade>>;
pub fn get_multi_trades(stock_symbols: &[&str], query: Option<&str>) -> MultiTrades {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/trades?symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct Res {
        trades: Option<MultiTrades>,
        next_page_token: Option<String>,
    }
    let mut trades_map: MultiTrades = HashMap::new();
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
                for (symbol, bars) in r.trades.unwrap() {
                    trades_map.entry(symbol).or_insert(Vec::new()).extend(bars);
                }
            }
            _ => {
                for (symbol, bars) in r.trades.unwrap() {
                    trades_map.entry(symbol).or_insert(Vec::new()).extend(bars);
                }
                break;
            }
        }
    }
    trades_map
}

pub fn get_latest_trade(stock_symbol: &str) -> Trade {
    let address = format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/trades/latest");

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Res {
        trade: Option<Trade>,
        symbol: String,
    }
    let r: Res = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    r.trade.expect("No Trade In Response")
}

pub type MultiLatestTrades = HashMap<String, Trade>;
pub fn get_multi_latest_trades(stock_symbols: &[&str], query: Option<&str>) -> MultiLatestTrades {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/trades/latest?symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct Res {
        trades: Option<MultiLatestTrades>,
    }

    let r: Res = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    r.trades.expect("No Trades In Response")
}
