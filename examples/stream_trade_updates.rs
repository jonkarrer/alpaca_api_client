use alpaca_api_client::stream::TradeUpdateStream;
use alpaca_api_client::trading::AccountType;

fn main() {
    println!("Connecting to trade updates stream...");

    TradeUpdateStream::new(AccountType::Paper)
        .start(|update| {
            println!(
                "[{}] {} - {} @ {}",
                update.event,
                update.order.symbol,
                update.order.side,
                update.price.unwrap_or_default()
            );
        })
        .unwrap();
}
