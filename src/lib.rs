use dotenvy::dotenv;

pub mod market_data;
pub mod trading;

mod timeframe;
pub use timeframe::TimeFrame;

mod trend;
pub use trend::Trend;

mod stream;
pub use stream::*;

mod serde;

fn get_auth() -> (String, String) {
    dotenv().ok();
    let id_key = std::env::var("APCA_API_KEY_ID").expect("API Id Key Not Found");
    let secret_key = std::env::var("APCA_API_SECRET_KEY").expect("API Secret Key Not Found");
    (id_key, secret_key)
}

/// For requests without a body (GET, DELETE)
fn request(method: &str, address: &str) -> ureq::RequestBuilder<ureq::typestate::WithoutBody> {
    let (id_key, secret_key) = get_auth();

    match method {
        "GET" => ureq::get(address),
        "DELETE" => ureq::delete(address),
        _ => panic!("Use json_request for methods with body: {}", method),
    }
    .header("APCA-API-KEY-ID", &id_key)
    .header("APCA-API-SECRET-KEY", &secret_key)
}

/// For requests with a body (POST, PUT, PATCH)
fn json_request(method: &str, address: &str) -> ureq::RequestBuilder<ureq::typestate::WithBody> {
    let (id_key, secret_key) = get_auth();

    match method {
        "POST" => ureq::post(address),
        "PUT" => ureq::put(address),
        "PATCH" => ureq::patch(address),
        _ => panic!("Use request for methods without body: {}", method),
    }
    .header("APCA-API-KEY-ID", &id_key)
    .header("APCA-API-SECRET-KEY", &secret_key)
}
