use alpaca_api_client::trading::{
    order::{delete_all_orders, delete_by_id},
    AccountType,
};

fn main() {
    // Delete order by id
    delete_by_id("52fec271-0b23-4f79-8ab4-97e9981879fc", AccountType::Paper).unwrap();

    // Delete all orders
    delete_all_orders(AccountType::Paper).unwrap();
}
