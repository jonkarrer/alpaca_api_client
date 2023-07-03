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

/// Get the trades for a specific stock with a query option
pub fn get_trades(stock_symbol: &str, query: Option<&str>) -> Result<Vec<Trade>, ureq::Error> {
    let url = format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/trades");
    let address = match query {
        Some(query) => format!("{url}?{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct TradesResponse {
        trades: Vec<Trade>,
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
        let response = request("GET", &temp_address).call()?;
        let response: TradesResponse = response.into_json()?;
        trades.extend(response.trades);

        // If a token is in response, assign to page_token for next loop
        match response.next_page_token {
            Some(next_page_token) => page_token = Some(next_page_token.clone()),

            _ => break,
        }
    }

    Ok(trades)
}

pub type MultiTrades = HashMap<String, Vec<Trade>>;

/// Get multiple trades for multiple stocks
pub fn get_multi_trades(
    stock_symbols: &[&str],
    query: Option<&str>,
) -> Result<MultiTrades, ureq::Error> {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/trades?symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct MultiTradesResponse {
        trades: MultiTrades,
        next_page_token: Option<String>,
    }
    let mut multi_trades: MultiTrades = HashMap::new();
    let mut page_token = None;

    loop {
        // If a token exists, append to address
        let temp_address = match page_token {
            Some(token) => format!("{}&page_token={}", &address, &token),
            _ => address.clone(),
        };
        let response = request("GET", &temp_address).call()?;
        let response: MultiTradesResponse = response.into_json()?;

        for (symbol, bars) in response.trades {
            multi_trades
                .entry(symbol)
                .or_insert(Vec::new())
                .extend(bars);
        }

        // If a token is in response, assign to page_token for next loop
        match response.next_page_token {
            Some(next_page_token) => page_token = Some(next_page_token.clone()),
            _ => break,
        }
    }

    Ok(multi_trades)
}

/// Get latest trade for single stock
pub fn get_latest_trade(stock_symbol: &str, query: Option<&str>) -> Result<Trade, ureq::Error> {
    let url = format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/trades/latest");

    let address = match query {
        Some(query) => format!("{url}?{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    pub struct LatestTradeResponse {
        pub trade: Trade,
        pub symbol: String,
    }
    let response = request("GET", &address).call()?;
    let latest_trade: LatestTradeResponse = response.into_json()?;

    Ok(latest_trade.trade)
}

pub type MultiLatestTrades = HashMap<String, Trade>;

/// Get latest trade for multiple stocks
pub fn get_multi_latest_trades(
    stock_symbols: &[&str],
    query: Option<&str>,
) -> Result<MultiLatestTrades, ureq::Error> {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/trades/latest?symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    pub struct MultiLatestTradesResponse {
        pub trades: MultiLatestTrades,
    }
    let response = request("GET", &address).call()?;
    let response: MultiLatestTradesResponse = response.into_json()?;

    Ok(response.trades)
}
