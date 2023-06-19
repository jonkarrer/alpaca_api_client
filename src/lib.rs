//! # Alpaca API Client (unofficial)
//!<div align="center">
//!  <!-- Version -->
//!  <a href="https://crates.io/crates/alpaca_api_client">
//!    <img src="https://img.shields.io/crates/v/alpaca_api_client.svg?style=flat-square"
//!    alt="Crates.io version" />
//!  </a>
//!  <!-- Docs -->
//!  <a href="https://docs.rs/alpaca_api_client">
//!    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
//!      alt="docs.rs docs" />
//!  </a>
//!  <!-- Downloads -->
//!  <a href="https://crates.io/crates/alpaca_api_client">
//!    <img src="https://img.shields.io/crates/d/alpaca_api_client.svg?style=flat-square"
//!      alt="Crates.io downloads" />
//!  </a>
//!</div>
//!
//! **DO NOT USE IN A SERIOUS PROJECT, NOT LIABLE FOR ANY ISSUES**
//!
//! ## Introduction
//!
//! <a href="https://alpaca.markets/">Alpaca</a> is a trading platform for developers and app makers,
//! and they provide various endpoints to access over http. The goal of this package is to provide
//! the bare minimum tools for using the Alpaca API, and to learn more about open source software building.
//! This is not a batteries included package yet, and still requires a lot of testing.
//!
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
//! let multi_bars: MultiBars = get_multi_bars(&watchlist, "1Day", Some("start=2022-01-01"));
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
