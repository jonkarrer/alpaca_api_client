use super::request;
use serde::Deserialize;
use std::collections::HashMap;

/// Trend enum
#[derive(Debug, PartialEq, Clone)]
pub enum Trend {
    Bullish,
    Bearish,
}

/// API object for a Bar
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

/// Get the trend of the bar i.e Bull/Bear
impl Bar {
    pub fn trend(&self) -> Trend {
        let signal = self.c - self.o;

        if signal > 0.0 {
            return Trend::Bullish;
        } else {
            return Trend::Bearish;
        }
    }
}

/// A custom type for a vector of bars
pub type Bars = Vec<Bar>;

/// A custom type for a hashmap of stock symbols and bars
pub type MultiBars = HashMap<String, Bars>;

/// Utility methods for interacting with Bars type
pub trait BarSession {
    fn get_opens(&self) -> Vec<f32>;
    fn get_closes(&self) -> Vec<f32>;
    fn get_highs(&self) -> Vec<f32>;
    fn get_lows(&self) -> Vec<f32>;
    fn get_last_bar(&self) -> &Bar;
}

impl BarSession for Vec<Bar> {
    fn get_opens(&self) -> Vec<f32> {
        let mut opens = Vec::new();
        for item in self {
            opens.push(item.o);
        }
        opens
    }
    fn get_closes(&self) -> Vec<f32> {
        let mut closes = Vec::new();
        for item in self {
            closes.push(item.c);
        }
        closes
    }
    fn get_highs(&self) -> Vec<f32> {
        let mut highs = Vec::new();
        for item in self {
            highs.push(item.h);
        }
        highs
    }
    fn get_lows(&self) -> Vec<f32> {
        let mut lows = Vec::new();
        for item in self {
            lows.push(item.l);
        }
        lows
    }
    fn get_last_bar(&self) -> &Bar {
        &self[&self.len() - 1]
    }
}

/// Get bars for a single stock
pub fn get_bars(
    stock_symbol: &str,
    timeframe: &str,
    query: Option<&str>,
) -> Result<Bars, ureq::Error> {
    let url =
        format!("https://data.alpaca.markets/v2/stocks/{stock_symbol}/bars?timeframe={timeframe}");
    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct BarsResponse {
        bars: Bars,
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
        let response = request("GET", &temp_address).call()?;
        let response: BarsResponse = response.into_json()?;

        // Add bars to collection
        bars.extend(response.bars);

        // If a token is in response, assign to page_token for next loop
        match response.next_page_token {
            Some(next_page_token) => page_token = Some(next_page_token.clone()),
            _ => break,
        }
    }

    Ok(bars)
}

/// Get bars for multiple stocks at a time
pub fn get_multi_bars(
    stock_symbols: &[&str],
    timeframe: &str,
    query: Option<&str>,
) -> Result<MultiBars, ureq::Error> {
    let url = format!(
        "https://data.alpaca.markets/v2/stocks/bars?timeframe={timeframe}&symbols={}",
        stock_symbols.join(",")
    );

    let address = match query {
        Some(query) => format!("{url}&{query}"),
        _ => format!("{url}"),
    };

    #[derive(Deserialize)]
    struct MultiBarsResponse {
        bars: MultiBars,
        next_page_token: Option<String>,
    }

    let mut multi_bars: MultiBars = HashMap::new();
    let mut page_token = None;

    loop {
        // If a token exists, append to address
        let temp_address = match page_token {
            Some(token) => format!("{}&page_token={}", &address, &token),
            _ => address.clone(),
        };
        let response = request("GET", &temp_address).call()?;
        let response: MultiBarsResponse = response.into_json()?;

        // Add multi_bars to collection
        for (symbol, bars) in response.bars {
            multi_bars.entry(symbol).or_insert(Vec::new()).extend(bars);
        }

        // If a token is in response, assign to page_token for next loop
        match response.next_page_token {
            Some(next_page_token) => page_token = Some(next_page_token.clone()),
            _ => break,
        }
    }

    Ok(multi_bars)
}
