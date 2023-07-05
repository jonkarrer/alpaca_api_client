# Alpaca API Client in Rust

**Still a Work In Progress**
Do Not Use This Package In Any Serious Capacity Yet. Not Liable for Any Issues.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)
![Version 0.3.1](https://img.shields.io/badge/version-0.3.1-blue.svg)

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Contribution](#contribution)
- [License](#license)

## Introduction

<a href="https://alpaca.markets/">Alpaca</a> is a trading platform for developers and app makers,
and they provide various endpoints to access over http. The goal of this package is to provide
the bare minimum tools for using the Alpaca API.

Still exploring Rust and open source development, so this package may not be as professional or robust as other libraries out there. I am committed to creating and maintaining this library to the best of my ability, and do use this daily for stock trading.

## Features

- **Safe and single threaded**
- **Easy to use**
- **Minimal overhead**

## Installation

To install the Alpaca API Client, you will need Rust installed on your machine. If you don't have Rust installed, you can follow the [official guide](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install the Alpaca API Client using cargo:

```bash
cargo install alpaca_api_client
```

Add your API keys to an <b>.env</b> file in the root of your directory with these names.

```bash
// /.env

APCA_API_KEY_ID=<pub_key>
APCA_API_SECRET_KEY=<secret_key>
```

## Usage

[RS Docs](https://docs.rs/alpaca_api_client/0.3.1/alpaca_api_client/)

Get bars for a single stock

```rust
use alpaca_api_client::get_bars;

let bars = get_bars("BTU", "1Day", Some("start=2023-02-23")).unwrap();
```

Get bars for multiple symbols

```rust
use alpaca_api_client::get_multi_bars;

let watchlist: [&str; 30] = [
  "META", "DIS", "CMCSA", "VZ", "T", "CHTR", "NFLX", "TMUS", "TWTR", "FOXA", "FOX", "DISH",
  "CBS", "OMC", "TME", "TTWO", "EA", "ATVI", "ZM", "MTCH", "IAC", "NTES", "BIDU", "ROKU", "SPOT",
  "LYV", "IQ", "HUYA", "DOYU", "VIAV",
];

let mut multi_bars = match get_multi_bars(&watchlist, "1Day", Some("start=2023-01-01")) {
    Ok(multi_bars_map) => multi_bars_map,
    Err(e) => {
        println!("MultiBar Request Error:{}", e);
        return None;
    }
};
```

Place market order

```rust
use alpaca_api_client::{place_market_order, OrderSide};

let order = place_market_order("SO", 3.0, OrderSide::Buy).unwrap();
```

## Contribution

Any and all PR's are welcome. This is my first Rust project and my first foray into open source. I see a need for this type of Rust client to support Alpaca's v2 API.

## License

This project is licensed under the MIT and APACHE License.
