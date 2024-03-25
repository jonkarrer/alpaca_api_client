# Alpaca API Client in Rust

**Still a Work In Progress**
Not Production Ready. Not Liable for Any Issues. Recommended for Paper Accounts only.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)
![Version 0.5.0](https://img.shields.io/badge/version-0.3.2-blue.svg)

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Contribution](#contribution)
- [License](#license)

## Introduction

<a href="https://alpaca.markets/">Alpaca</a> is a trading platform for developers and app makers,
and they provide various endpoints to access over http. The goal of this package is to provide
the bare minimum tools for using the Alpaca API.

## Features

- **Get Stock Bars**
- **Get Stock Trades**
- **Get Positions**
- **Place Orders**
- **View Account**
- **View Activity**

## Installation

To install the Alpaca API Client, you will need Rust installed on your machine. If you don't have Rust installed, you can follow the [official guide](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can install the Alpaca API Client using cargo:

```bash
cargo install alpaca_api_client
```

Add your API keys to an <b>.env</b> file in the root of your directory with these names.

```bash
/.env

APCA_API_KEY_ID=<pub_key>
APCA_API_SECRET_KEY=<secret_key>
```

## Usage

[RS Docs](https://docs.rs/alpaca_api_client/0.3.2/alpaca_api_client/)

Get Bars for a single stock

```rust
use alpaca_api_client::get_bars;

// Args(symbol, timeframe, query)
let bars = get_bars("BTU", "1Day", Some("start=2023-02-23")).unwrap();
```

Get Bars for multiple symbols

```rust
use alpaca_api_client::{get_multi_bars, MultiBars};

let watchlist: [&str; 3] = ["META", "DIS", "VZ"];

let multiple_bars = get_multi_bars(&watchlist, "1Day", Some("start=2023-01-01")).unwrap();
```

Place Market order

```rust
use alpaca_api_client::{place_market_order, OrderSide};

// Args(symbol, quantity, side)
let order = place_market_order("SO", 3.0, OrderSide::Buy).unwrap();
```

Place Bracket order

```rust
use alpaca_api_client::{place_bracket_order, OrderSide};

// Args(symbol, quantity, side, take_profit, stop_loss)
let order = place_bracket_order("ABBV", 3.0, OrderSide::Buy, 170.00, 120.00).unwrap();
```

## Contribution

Any and all PR's are welcome. I see a need for this type of Rust client to support Alpaca's v2 API.

## License

This project is licensed under the MIT and APACHE License.
