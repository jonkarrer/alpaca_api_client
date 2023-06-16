use dotenv::dotenv;
use ureq::Request;

mod account;
pub use self::account::*;

mod activity;
pub use self::activity::*;

mod bars;
pub use self::bars::*;

mod order;
pub use self::order::*;

mod positions;
pub use self::positions::*;

mod trades;
pub use self::trades::*;

fn request(method: &str, address: &str) -> Request {
    dotenv().ok();
    let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");

    ureq::request(method, address)
        .set("APCA-API-KEY-ID", &id_key)
        .set("APCA-API-SECRET-KEY", &secret_key)
}
