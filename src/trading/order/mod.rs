pub mod create;
pub mod delete;
pub mod get;
pub mod replace;

use serde::{Deserialize, Serialize};

pub type AllOrders = Vec<Order>;

/// API object for an Order
#[derive(Deserialize, Serialize, Debug)]
pub struct Order {
    pub id: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub submitted_at: Option<String>,
    pub filled_at: Option<String>,
    pub expired_at: Option<String>,
    pub canceled_at: Option<String>,
    pub failed_at: Option<String>,
    pub replaced_at: Option<String>,
    pub replaced_by: Option<String>,
    pub replaces: Option<String>,
    pub asset_id: Option<String>,
    pub symbol: String,
    pub asset_class: Option<String>,
    pub notional: Option<String>,
    pub qty: Option<String>,
    pub filled_qty: Option<String>,
    pub filled_avg_price: Option<String>,
    pub order_class: Option<String>,
    pub order_type: String,
    pub r#type: String,
    pub side: String,
    pub time_in_force: Option<String>,
    pub limit_price: Option<String>,
    pub stop_price: Option<String>,
    pub status: String,
    pub extended_hours: bool,
    pub legs: Option<Vec<Self>>,
    pub trail_percent: Option<String>,
    pub trail_price: Option<String>,
    pub hwm: Option<String>,
}
