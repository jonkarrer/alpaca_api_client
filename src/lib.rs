//! # Alpaca API Client (unofficial)
//!
//! <a href="https://alpaca.markets/">Alpaca</a> is a trading platform for developers and app makers
//!
//! **DO NOT USE IN A SERIOUS PROJECT, NOT LIABLE FOR ANY ISSUES**
//!
//! Provides:
//! * Quick access to SOME of Alpaca's API endpoints
//!
//! ## Quick Examples
//!
//! Get bars for a single stock
//!
//! ```rust
//! use alpaca_api_client::get_bars;
//!
//! let bars = get_bars("BTU", "1Day", Some("start=2023-02-23"));
//! ```
//!
//! Get bars for multiple symbols
//!
//! ```rust
//! use alpaca_api_client::get_multi_bars;
//!
//! let watchlist: [&str; 30] = [
//!   "META", "DIS", "CMCSA", "VZ", "T", "CHTR", "NFLX", "TMUS", "TWTR", "FOXA", "FOX", "DISH",
//!   "CBS", "OMC", "TME", "TTWO", "EA", "ATVI", "ZM", "MTCH", "IAC", "NTES", "BIDU", "ROKU", "SPOT",
//!   "LYV", "IQ", "HUYA", "DOYU", "VIAV",
//! ];
//!
//! let multi_bars: MultiBars = get_multi_bars(watchlist, "1Day", Some("start=2022-01-01"));
//! ```
//!
//! Place market order
//!
//! ```rust
//! use alpaca_api_client::{place_market_order, OrderSide};
//!
//! let order = place_market_order("SO", 3.0, OrderSide::Buy);
//! ```
//!

use dotenv::dotenv;
use ureq::Request;

mod account;
pub use self::account::*;

mod activity;
pub use self::activity::*;

mod bars;
pub use self::bars::*;

mod order;
pub use self::order::*;

mod positions;
pub use self::positions::*;

mod trades;
pub use self::trades::*;

fn request(method: &str, address: &str) -> Request {
    dotenv().ok();
    let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");

    ureq::request(method, address)
        .set("APCA-API-KEY-ID", &id_key)
        .set("APCA-API-SECRET-KEY", &secret_key)
}
