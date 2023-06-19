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
    struct Res {
        trades: Option<Vec<Trade>>,
        //symbol: String,
        //next_page_token: Option<String>,
    }
    let r: Res = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    r.trades.expect("No Trades In Response")
}

pub fn get_latest_trade(stock_symbol: &str) -> Trade {
    let address = format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/trades/latest");

    #[derive(Deserialize)]
    struct Res {
        trade: Option<Trade>,
        //symbol: String,
    }
    let r: Res = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    r.trade.expect("No Trade In Response")
}

pub type MultiTrades = HashMap<String, Trade>;

pub fn get_multi_latest_trades(stock_symbols: &[&str], query: Option<&str>) -> MultiTrades {
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
        trades: Option<MultiTrades>,
    }

    let r: Res = request("GET", &address)
        .call()
        .expect("Could Not Call API")
        .into_json()
        .expect("Could Not Parse Response Into Json");

    r.trades.expect("No Trades In Response")
}
