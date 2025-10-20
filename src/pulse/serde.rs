use super::types::Charts;
use serde_json;

// simple conversion from json body to chart struct
pub fn parse_charts(body: &str) -> serde_json::Result<Charts> {
    serde_json::from_str(body)
}
