# Alpaca API Client for Rust

An unofficial Rust SDK for the [Alpaca](https://alpaca.markets/) trading API. This library provides a type-safe, ergonomic interface for accessing Alpaca's Market Data and Trading APIs.

![Build Status](https://img.shields.io/badge/build-passing-green.svg) ![Version 0.6.4](https://img.shields.io/badge/version-0.6.4-blue.svg) ![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)

> **Note:** Recommended for Paper Accounts. Use with live accounts at your own risk.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Quick Start](#quick-start)
- [Market Data API](#market-data-api)
  - [Stocks](#stocks)
  - [Crypto](#crypto)
  - [Options](#options)
  - [News](#news)
  - [Screener](#screener)
- [Trading API](#trading-api)
  - [Orders](#orders)
  - [Positions](#positions)
  - [Account](#account)
  - [Portfolio](#portfolio)
  - [Assets](#assets)
  - [Clock & Calendar](#clock--calendar)
  - [Activities](#activities)
- [Types & Enums](#types--enums)
- [Error Handling](#error-handling)
- [Contributing](#contributing)
- [License](#license)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
alpaca_api_client = "0.6"
```

Or use cargo:

```bash
cargo add alpaca_api_client
```

## Configuration

The library uses environment variables for API authentication. Create a `.env` file in your project root:

```bash
APCA_API_KEY_ID=your_api_key_id
APCA_API_SECRET_KEY=your_api_secret_key
```

Or set them directly in your environment. The library uses `dotenvy` to load these automatically.

## Quick Start

```rust
use alpaca_api_client::{
    trading::{AccountType, order::{CreateOrderQuery, OrderSide, OrderType, TimeInForce}},
    market_data::stocks::LatestBarsQuery,
};

fn main() -> Result<(), ureq::Error> {
    // Get latest stock prices
    let bars = LatestBarsQuery::new(vec!["AAPL", "GOOGL"])
        .feed("iex")
        .send()?;

    println!("AAPL price: {:?}", bars.get("AAPL"));

    // Place a market order
    let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::Day)
        .qty("1")
        .send(AccountType::Paper)?;

    println!("Order placed: {}", order.id);

    Ok(())
}
```

---

## Market Data API

All market data queries follow a builder pattern. Construct your query, chain optional parameters, then call `.send()`.

### Stocks

#### Historical Bars

```rust
use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame};

let bars = HistoricalBarsQuery::new(vec!["AAPL", "TSLA"], TimeFrame::OneDay)
    .start("2024-01-01")
    .end("2024-01-31")
    .feed("iex")        // "iex" (free) or "sip" (premium)
    .limit(100)
    .sort_desc()
    .send()?;

// Returns HashMap<String, Vec<StockBar>>
for (symbol, bars) in &bars {
    println!("{}: {} bars", symbol, bars.len());
}
```

#### Latest Bars

```rust
use alpaca_api_client::market_data::stocks::LatestBarsQuery;

let bars = LatestBarsQuery::new(vec!["AAPL", "TSLA"])
    .feed("iex")
    .send()?;

// Returns HashMap<String, StockBar>
if let Some(bar) = bars.get("AAPL") {
    println!("AAPL - Open: {}, High: {}, Low: {}, Close: {}", bar.o, bar.h, bar.l, bar.c);
}
```

#### Quotes

```rust
use alpaca_api_client::market_data::stocks::{HistoricalQuotesQuery, LatestQuotesQuery};

// Historical quotes
let quotes = HistoricalQuotesQuery::new(vec!["AAPL"])
    .start("2024-01-01")
    .limit(100)
    .send()?;

// Latest quotes
let latest = LatestQuotesQuery::new(vec!["AAPL", "GOOGL"])
    .feed("iex")
    .send()?;
```

#### Trades

```rust
use alpaca_api_client::market_data::stocks::{HistoricalTradesQuery, LatestTradesQuery};

// Historical trades
let trades = HistoricalTradesQuery::new(vec!["AAPL"])
    .start("2024-01-01")
    .limit(100)
    .send()?;

// Latest trades
let latest = LatestTradesQuery::new(vec!["AAPL"])
    .send()?;
```

#### Snapshots

Get a complete market snapshot including latest trade, quote, and bars:

```rust
use alpaca_api_client::market_data::stocks::SnapshotsQuery;

let snapshots = SnapshotsQuery::new(vec!["AAPL", "GOOGL"])
    .feed("iex")
    .send()?;

if let Some(snapshot) = snapshots.get("AAPL") {
    if let Some(trade) = &snapshot.latest_trade {
        println!("Latest trade price: {}", trade.p);
    }
}
```

#### Auctions

```rust
use alpaca_api_client::market_data::stocks::HistoricalAuctionsQuery;

let auctions = HistoricalAuctionsQuery::new(vec!["AAPL"])
    .start("2024-01-01")
    .feed("sip")
    .send()?;
```

### Crypto

Crypto market data uses similar patterns to stocks:

```rust
use alpaca_api_client::{
    market_data::crypto::{
        HistoricalCryptoBarsQuery, LatestCryptoBarsQuery,
        HistoricalCryptoTradesQuery, LatestCryptoTradesQuery,
        HistoricalCryptoQuotesQuery, LatestCryptoQuotesQuery,
        SnapshotsQuery, OrderbookQuery,
    },
    TimeFrame,
};

// Historical bars
let bars = HistoricalCryptoBarsQuery::new(vec!["BTC/USD", "ETH/USD"], TimeFrame::OneHour)
    .start("2024-01-01")
    .limit(100)
    .send()?;

// Latest bars
let latest = LatestCryptoBarsQuery::new(vec!["BTC/USD"])
    .send()?;

// Snapshots
let snapshots = SnapshotsQuery::new(vec!["BTC/USD"])
    .send()?;

// Order book
let orderbooks = OrderbookQuery::new(vec!["BTC/USD"])
    .send()?;

if let Some(book) = orderbooks.get("BTC/USD") {
    println!("Best bid: {:?}", book.b.first());
    println!("Best ask: {:?}", book.a.first());
}
```

### Options

```rust
use alpaca_api_client::{
    market_data::options::{
        HistoricalOptionBarsQuery, HistoricalOptionTradesQuery,
        LatestOptionTradesQuery, LatestOptionQuotesQuery,
        OptionSnapshotQuery, OptionChainQuery,
    },
    TimeFrame,
};

// Option symbols follow OCC format: AAPL261218C00200000
// (Underlying + YYMMDD + C/P + Strike*1000)

// Historical bars
let bars = HistoricalOptionBarsQuery::new(vec!["AAPL261218C00200000"], TimeFrame::OneDay)
    .send()?;

// Latest quotes
let quotes = LatestOptionQuotesQuery::new(vec!["AAPL261218C00200000"])
    .feed("indicative")
    .send()?;

// Option snapshots with Greeks
let snapshots = OptionSnapshotQuery::new(vec!["AAPL261218C00200000"])
    .send()?;

if let Some(snap) = snapshots.get("AAPL261218C00200000") {
    if let Some(greeks) = &snap.greeks {
        println!("Delta: {}, Gamma: {}, Theta: {}", greeks.delta, greeks.gamma, greeks.theta);
    }
}

// Option chain for underlying
let chain = OptionChainQuery::new("AAPL")
    .expiration_date_gte("2024-06-01")
    .expiration_date_lte("2024-12-31")
    .strike_price_gte(150.0)
    .strike_price_lte(250.0)
    .set_type("call")
    .limit(50)
    .send()?;
```

### News

```rust
use alpaca_api_client::market_data::news::NewsQuery;

let news = NewsQuery::new(vec!["AAPL", "TSLA"])
    .start("2024-01-01")
    .limit(10)
    .include_content(true)
    .exclude_contentless(true)
    .sort_desc()
    .send()?;

for article in &news {
    println!("{} - {}", article.created_at, article.headline);
}
```

### Screener

```rust
use alpaca_api_client::market_data::screener::{ActiveStocksQuery, TopMoversQuery, MarketType};

// Most active stocks
let actives = ActiveStocksQuery::new()
    .by("volume")  // or "trades"
    .top(10)
    .send()?;

for stock in &actives {
    println!("{}: {} volume", stock.symbol, stock.volume);
}

// Top movers (gainers and losers)
let movers = TopMoversQuery::new(MarketType::Stocks)
    .top(10)
    .send()?;

println!("Top Gainers:");
for gainer in &movers.gainers {
    println!("  {}: +{:.2}%", gainer.symbol, gainer.percent_change);
}

println!("Top Losers:");
for loser in &movers.losers {
    println!("  {}: {:.2}%", loser.symbol, loser.percent_change);
}
```

---

## Trading API

All trading operations require specifying `AccountType::Paper` or `AccountType::Live`.

### Orders

#### Create Orders

```rust
use alpaca_api_client::trading::{
    AccountType,
    order::{CreateOrderQuery, OrderSide, OrderType, OrderClass, TimeInForce, TakeProfit, StopLoss},
};

// Market order
let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::Day)
    .qty("10")
    .send(AccountType::Paper)?;

// Limit order
let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Limit, TimeInForce::GoodTilCanceled)
    .qty("10")
    .limit_price("150.00")
    .send(AccountType::Paper)?;

// Stop order
let order = CreateOrderQuery::new("AAPL", OrderSide::Sell, OrderType::Stop, TimeInForce::GoodTilCanceled)
    .qty("10")
    .stop_price("140.00")
    .send(AccountType::Paper)?;

// Stop-limit order
let order = CreateOrderQuery::new("AAPL", OrderSide::Sell, OrderType::StopLimit, TimeInForce::GoodTilCanceled)
    .qty("10")
    .stop_price("140.00")
    .limit_price("139.00")
    .send(AccountType::Paper)?;

// Trailing stop order
let order = CreateOrderQuery::new("AAPL", OrderSide::Sell, OrderType::TrailingStop, TimeInForce::GoodTilCanceled)
    .qty("10")
    .trail_percent("5")  // or .trail_price("10.00")
    .send(AccountType::Paper)?;

// Bracket order (entry with take-profit and stop-loss)
let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::GoodTilCanceled)
    .qty("10")
    .order_class(OrderClass::Bracket)
    .take_profit(TakeProfit::new("200.00"))
    .stop_loss(StopLoss::new("140.00", "139.00"))  // stop_price, limit_price
    .send(AccountType::Paper)?;

// One-Triggers-Other (OTO) order
let order = CreateOrderQuery::new("AAPL", OrderSide::Buy, OrderType::Market, TimeInForce::GoodTilCanceled)
    .qty("10")
    .order_class(OrderClass::OneTriggersOther)
    .stop_loss(StopLoss::new("140.00", "139.00"))
    .send(AccountType::Paper)?;
```

#### Get Orders

```rust
use alpaca_api_client::trading::{AccountType, order::GetOrdersQuery};

// Get all orders
let orders = GetOrdersQuery::new(AccountType::Paper)
    .status("open")  // "open", "closed", "all"
    .limit(100)
    .symbols(vec!["AAPL", "TSLA"])
    .side("buy")     // "buy" or "sell"
    .direction("desc")
    .send()?;

// Get order by ID
let order = GetOrdersQuery::new(AccountType::Paper)
    .get_by_id("order-uuid-here", true)?;  // true = include nested orders
```

#### Cancel Orders

```rust
use alpaca_api_client::trading::{AccountType, order::{delete_all_orders, delete_by_id}};

// Cancel all open orders
let results = delete_all_orders(AccountType::Paper)?;

// Cancel specific order
let status = delete_by_id("order-uuid-here", AccountType::Paper)?;
// Returns HTTP status code (204 on success)
```

#### Replace Orders

```rust
use alpaca_api_client::trading::{AccountType, order::{ReplaceOrderQuery, TimeInForce}};

let order = ReplaceOrderQuery::new("order-uuid-here")
    .qty("20")
    .limit_price("155.00")
    .time_in_force(TimeInForce::GoodTilCanceled)
    .send(AccountType::Paper)?;
```

### Positions

```rust
use alpaca_api_client::trading::{AccountType, positions::PositionsQuery};

let positions = PositionsQuery::new(AccountType::Paper);

// Get all open positions
let all = positions.get_all_open_positions()?;

for pos in &all {
    println!("{}: {} shares, P&L: {}", pos.symbol, pos.qty, pos.unrealized_pl);
}

// Get position by symbol
let pos = positions.get_position_by_symbol("AAPL")?;

// Close all positions
let closed = positions.close_all_positions(true)?;  // true = cancel open orders

// Close specific position
let order = positions.close_position_by_id_or_symbol(
    "AAPL",
    Some(5.0),   // qty to close (optional)
    None,        // percentage to close (optional)
)?;
```

### Account

```rust
use alpaca_api_client::trading::{AccountType, account::{get_account, get_account_configurations, PatchAccountConfigQuery}};

// Get account info
let account = get_account(AccountType::Paper)?;
println!("Buying power: {}", account.buying_power);
println!("Portfolio value: {}", account.portfolio_value);
println!("Cash: {}", account.cash);

// Get account configuration
let config = get_account_configurations(AccountType::Paper)?;

// Update account configuration
let new_config = PatchAccountConfigQuery::new()
    .fractional_trading(true)
    .no_shorting(false)
    .send(AccountType::Paper)?;
```

### Portfolio

```rust
use alpaca_api_client::{trading::{AccountType, portfolio::PortfolioHistoryQuery}, TimeFrame};

let history = PortfolioHistoryQuery::new(AccountType::Paper)
    .period("1M")  // 1D, 1W, 1M, 3M, 1A, all
    .timeframe(TimeFrame::OneDay)
    .extended_hours("true")
    .send()?;

for (i, timestamp) in history.timestamp.iter().enumerate() {
    println!("Time: {}, Equity: {}, P&L: {}",
        timestamp,
        history.equity[i],
        history.profit_loss[i]
    );
}
```

### Assets

```rust
use alpaca_api_client::trading::{AccountType, assets::{AssetsQuery, OptionContractsQuery}};

// Get all tradable assets
let assets = AssetsQuery::new(AccountType::Paper)
    .status("active")
    .asset_class("us_equity")
    .send()?;

// Get specific asset
let asset = AssetsQuery::new(AccountType::Paper)
    .get_by_symbol("AAPL")?;

println!("{} - Tradable: {}, Fractionable: {}", asset.symbol, asset.tradable, asset.fractionable);

// Get option contracts
let contracts = OptionContractsQuery::new(AccountType::Paper)
    .underlying_symbols(vec!["AAPL"])
    .expiration_date_gte("2024-06-01")
    .limit(50)
    .send()?;
```

### Clock & Calendar

```rust
use alpaca_api_client::trading::{AccountType, clock::get_market_clock, calendar::CalendarQuery};

// Get market clock
let clock = get_market_clock(AccountType::Paper)?;
println!("Market is {}", if clock.is_open { "OPEN" } else { "CLOSED" });
println!("Next open: {}", clock.next_open);
println!("Next close: {}", clock.next_close);

// Get market calendar
let calendar = CalendarQuery::new(AccountType::Paper)
    .start("2024-01-01")
    .end("2024-12-31")
    .send()?;

for day in &calendar {
    println!("{}: {} - {}", day.date, day.open, day.close);
}
```

### Activities

```rust
use alpaca_api_client::trading::{AccountType, activities::ActivitiesQuery};

let activities = ActivitiesQuery::new(AccountType::Paper)
    .activity_types(vec!["FILL", "TRANS"])
    .after("2024-01-01")
    .direction("desc")
    .limit(100)
    .send()?;

for activity in &activities {
    println!("{:?}: {} {:?}", activity.activity_type, activity.symbol.as_deref().unwrap_or("N/A"), activity.qty);
}
```

---

## Types & Enums

### TimeFrame

```rust
use alpaca_api_client::TimeFrame;

// Available timeframes
TimeFrame::OneMinute      // "1Min"
TimeFrame::FiveMinutes    // "5Min"
TimeFrame::FifteenMinutes // "15Min"
TimeFrame::ThirtyMinutes  // "30Min"
TimeFrame::OneHour        // "1H"
TimeFrame::FourHours      // "4H"
TimeFrame::OneDay         // "1D"
TimeFrame::OneWeek        // "1W"
TimeFrame::OneMonth       // "1M"
```

### Order Enums

```rust
use alpaca_api_client::trading::order::{OrderSide, OrderType, TimeInForce, OrderClass};

// Order sides
OrderSide::Buy
OrderSide::Sell

// Order types
OrderType::Market
OrderType::Limit
OrderType::Stop
OrderType::StopLimit
OrderType::TrailingStop

// Time in force
TimeInForce::Day              // Day order
TimeInForce::GoodTilCanceled  // GTC
TimeInForce::OpeningOrder     // OPG - execute at market open
TimeInForce::ClosingOrder     // CLS - execute at market close
TimeInForce::ImmediateOrCancel // IOC
TimeInForce::FillOrKill       // FOK

// Order classes
OrderClass::Simple
OrderClass::Bracket
OrderClass::OneCancelsOther
OrderClass::OneTriggersOther
```

### Account Type

```rust
use alpaca_api_client::trading::AccountType;

AccountType::Paper  // Paper trading (sandbox)
AccountType::Live   // Live trading (real money)
```

### StreamBar (for WebSocket integration)

```rust
use alpaca_api_client::StreamBar;

// Used for parsing WebSocket bar data
pub struct StreamBar {
    pub bar_type: String,
    pub symbol: String,
    pub o: f32,    // open
    pub h: f32,    // high
    pub l: f32,    // low
    pub c: f32,    // close
    pub v: u32,    // volume
    pub t: String, // timestamp
    pub n: u32,    // number of trades
    pub vw: f32,   // volume weighted average
}
```

---

## Error Handling

All API calls return `Result<T, ureq::Error>`. Handle errors appropriately:

```rust
use alpaca_api_client::trading::{AccountType, order::GetOrdersQuery};

match GetOrdersQuery::new(AccountType::Paper).send() {
    Ok(orders) => {
        println!("Found {} orders", orders.len());
    }
    Err(e) => {
        eprintln!("API error: {}", e);
    }
}
```

Common error scenarios:
- `StatusCode(401)` - Invalid API credentials
- `StatusCode(403)` - Forbidden (insufficient permissions)
- `StatusCode(404)` - Resource not found
- `StatusCode(422)` - Invalid request parameters
- `StatusCode(429)` - Rate limited

---

## Contributing

Contributions are welcome! Areas that need work:

- Broker API implementation
- WebSocket streaming support
- Additional documentation and examples

Please submit PRs to [GitHub](https://github.com/jonkarrer/alpaca_api_client).

## License

This project is dual-licensed under MIT and Apache 2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).

---

## Resources

- [Alpaca Documentation](https://docs.alpaca.markets/)
- [Market Data API Docs](https://docs.alpaca.markets/docs/about-market-data-api)
- [Trading API Docs](https://docs.alpaca.markets/docs/trading-api)
- [API Reference on docs.rs](https://docs.rs/alpaca_api_client)
