use dotenvy::dotenv;
use ureq::Request;

pub mod market_data;
pub mod trading;

mod timeframe;
pub use timeframe::TimeFrame;

mod trend;
pub use trend::Trend;

fn request(method: &str, address: &str) -> Request {
    dotenv().ok();
    let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");

    ureq::request(method, address)
        .set("APCA-API-KEY-ID", &id_key)
        .set("APCA-API-SECRET-KEY", &secret_key)
}
