use reqwest::{Client, header::HeaderMap, header::HeaderValue};

// originally added before realizing headers need to be paired to the requestbuilder
pub fn new_client() -> Client {
    Client::new()
}

// headers for auth
pub fn new_headers(pulse_token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    match HeaderValue::from_str(pulse_token) {
        Ok(value) => {
            //token = value;
            headers.insert("X-API-Token", value);
        }
        Err(error) => {
            println!("Error setting header: {}", error)
        }
    };
    headers
}
