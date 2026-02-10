use serde::Deserialize;

use crate::{request, trading::AccountType};

#[derive(Deserialize, Debug)]
pub struct DeleteOrderResult {
    pub id: String,
    pub status: i32,
}

pub fn delete_all_orders(account_type: AccountType) -> Result<Vec<DeleteOrderResult>, ureq::Error> {
    let url = match account_type {
        AccountType::Live => "https://api.alpaca.markets/v2/orders",
        AccountType::Paper => "https://paper-api.alpaca.markets/v2/orders",
    };
    let response = request("DELETE", url).call()?;
    let orders: Vec<DeleteOrderResult> = response.into_body().read_json()?;
    Ok(orders)
}

/// Returns the HTTP status code on success
pub fn delete_by_id(id: &str, account_type: AccountType) -> Result<u16, ureq::Error> {
    let url = match account_type {
        AccountType::Live => format!("https://api.alpaca.markets/v2/orders/{}", id),
        AccountType::Paper => format!("https://paper-api.alpaca.markets/v2/orders/{}", id),
    };
    let response = request("DELETE", &url).call()?;
    Ok(response.status().as_u16())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_all_orders() {
        //! Will fail if there are no orders
        let res = delete_all_orders(AccountType::Paper).unwrap();
        dbg!(&res);
        assert!(res.len() > 0);
    }

    #[test]
    fn test_delete_order_by_id() {
        //! Will fail if the id is not found
        let status = delete_by_id("52fec271-0b23-4f79-8ab4-97e9981879fc", AccountType::Paper).unwrap();
        assert!(status == 204);
    }
}
