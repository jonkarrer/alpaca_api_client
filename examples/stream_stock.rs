use alpaca_api_client::stream::{Feed, MarketDataMessage, StockStream};

fn main() {
    println!("Connecting to stock stream...");

    StockStream::new(Feed::Iex)
        .subscribe_trades(vec!["AAPL", "TSLA"])
        .subscribe_bars(vec!["SPY"])
        .start(|msg| match msg {
            MarketDataMessage::Trade(t) => {
                println!("[TRADE] {} ${} x{}", t.symbol, t.p, t.s);
            }
            MarketDataMessage::Bar(b) => {
                println!(
                    "[BAR] {} O={} H={} L={} C={} V={}",
                    b.symbol, b.o, b.h, b.l, b.c, b.v
                );
            }
            _ => {}
        })
        .unwrap();
}
