use alpaca_api_client::stream::{CryptoStream, MarketDataMessage};

fn main() {
    println!("Connecting to crypto stream...");

    CryptoStream::new()
        .subscribe_trades(vec!["BTC/USD", "ETH/USD"])
        .start(|msg| match msg {
            MarketDataMessage::Trade(t) => {
                println!("[TRADE] {} ${}", t.symbol, t.p);
            }
            _ => {}
        })
        .unwrap();
}
