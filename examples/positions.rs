use alpaca_api_client::trading::{positions::PositionsQuery, AccountType};

fn main() {
    let position_query = PositionsQuery::new(AccountType::Paper);

    // Get all open positions
    position_query.get_all_open_positions().unwrap();

    // Get position by symbol
    position_query.get_position_by_symbol("AAPL").unwrap();

    // Get position by id
    position_query
        .get_position_by_id("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415")
        .unwrap();

    // Close all positions
    position_query.close_all_positions(true).unwrap();

    // Close position by id
    position_query
        .close_position_by_id_or_symbol("b0b6dd9d-8b9b-48a9-ba46-b9d54906e415", Some(1.0), None)
        .unwrap();
}
