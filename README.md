# Alpaca API Client in Rust

Do Not Use This Package In Any Serious Capcity. Not Liable for Any Issues.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)
![Version 0.2.0](https://img.shields.io/badge/version-0.1.6-blue.svg)

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
the bare minimum tools for using the Alpaca API, and to learn more about open source software building.
This is not a batteries included package yet, and still requires a lot of testing.

## Features

- **Feature 1:** Safe and single threaded
- **Feature 2:** Easy to use
- **Feature 3:** Minimal overhead

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

[RS Docs](https://docs.rs/alpaca_api_client/0.2.0/alpaca_api_client/)

Get bars for a single stock

```rust
use alpaca_api_client::get_bars;

let bars = get_bars("BTU", "1Day", Some("start=2023-02-23"));
```

Get bars for multiple symbols

```rust
use alpaca_api_client::get_multi_bars;

let watchlist: [&str; 30] = [
  "META", "DIS", "CMCSA", "VZ", "T", "CHTR", "NFLX", "TMUS", "TWTR", "FOXA", "FOX", "DISH",
  "CBS", "OMC", "TME", "TTWO", "EA", "ATVI", "ZM", "MTCH", "IAC", "NTES", "BIDU", "ROKU", "SPOT",
  "LYV", "IQ", "HUYA", "DOYU", "VIAV",
];

let multi_bars: MultiBars = get_multi_bars(&watchlist, "1Day", Some("start=2022-01-01"));
```

Place market order

```rust
use alpaca_api_client::{place_market_order, OrderSide};

let order = place_market_order("SO", 3.0, OrderSide::Buy);
```

## Contribution

Any and all PR's are welcome. This is my first Rust project and my first foray into open source. I see a need for this type of Rust client to support Alpaca's v2 API.

## License

This project is licensed under the MIT and APACHE License.
