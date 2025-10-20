use reqwest::{Client, header::HeaderMap};

// returns body from path
// later can refactor if we want to do anything custom instead of ?
#[tokio::main]
pub async fn get(
    client: &Client,
    pulse_address: &str,
    headers: &HeaderMap,
    path: &str,
) -> Result<String, reqwest::Error> {
    let result = client
        .get(format!("{}{}", pulse_address, path))
        .headers(headers.clone())
        .send()
        .await?
        .text()
        .await?;
    Ok(result)
}
