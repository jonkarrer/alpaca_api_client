# Alpaca API Client in Rust

Do Not Use This Package In Any Serious Capcity. Untested and still unstable. A WORK IN PROGRESS.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)
![Version 0.1.5](https://img.shields.io/badge/version-0.1.5-blue.svg)

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Contribution](#contribution)
- [License](#license)

## Introduction

This project provides a Rust client for the Alpaca Trading API. It is designed to provide a fast, reliable, and efficient way to interact with Alpaca's trading platform. Still a WORK IN PROGRESS. Does not have all the endpoints for the Alpaca API.

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

Add your API keys to a .env file in the root of your directory with these names.

```bash
APCA_API_KEY_ID=
APCA_API_SECRET_KEY=
```

## Usage

Simply import what you need.

```rust
use alpaca_api_client::{get_multi_bars, MultiBars};

let watchlist: [&str; 30] = [
    "META", "DIS", "CMCSA", "VZ", "T", "CHTR", "NFLX", "TMUS", "TWTR", "FOXA", "FOX", "DISH",
    "CBS", "OMC", "TME", "TTWO", "EA", "ATVI", "ZM", "MTCH", "IAC", "NTES", "BIDU", "ROKU", "SPOT",
    "LYV", "IQ", "HUYA", "DOYU", "VIAV",
];

let multi_bars: MultiBars = get_multi_bars(watchlist, "1Day", Some("start=2022-01-01"));
```

## Contribution

Any and all PR's are welcome. This is my first Rust project and my first foray into open source. I see a need for this type of Rust client to support Alpaca's v2 API.

## License

This project is licensed under the MIT and APACHE License.
