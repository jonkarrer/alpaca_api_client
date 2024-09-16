# Alpaca API Client in Rust

**Still a Work In Progress**
Recommended for Paper Accounts only. Help needed to take this all the way.

![Build Status](https://img.shields.io/badge/build-passing-green.svg)
![Version 0.6.0](https://img.shields.io/badge/version-0.3.2-blue.svg)

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
the basic functionality for using the Alpaca API. This is a work in progress, and will need more contributors as the package grows.

## Features

Currently, this package only provides the ability to interact with the Market Data and Trading APIs. The Broker API is not yet supported.

### Market Data

Link to the documentation for the market data endpoints -> [Alpaca API Docs](https://docs.alpaca.markets/docs/about-market-data-api)

- Stocks
- Crypto
- News
- Options

### Trading

## Installation

```bash
cargo add alpaca_api_client
```

Add your API keys to an <b>.env</b> file in the root of your directory with these names.

```bash
/.env

APCA_API_KEY_ID=<pub_key>
APCA_API_SECRET_KEY=<secret_key>
```

## Usage

[RS Docs](https://docs.rs/alpaca_api_client/0.6.0/alpaca_api_client/)

## Contribution

If you would like to contribute to the project, PR's are welcome. I see a need for this type of Rust client to support Alpaca's v2 API. The Broker API is primarily where the help would be needed.

## License

This project is licensed under the MIT and APACHE License.
