use dotenv::dotenv;
use std::collections::HashMap;
pub mod pg;
pub mod pulse;
fn main() {
    // load up dotenv
    dotenv().ok();

    // env vars we care about for pulse
    let env_var_keys = [
        "PULSE_ADDRESS",
        "PULSE_TOKEN",
        "POSTGRES_PASSWORD",
        "POSTGRES_USER",
        "POSTGRES_HOST",
    ];

    // create a hashmap to store values in
    let mut pulse_vars = HashMap::new();

    // get values for vars and store in map, default to empty if not found
    for key in env_var_keys {
        match dotenv::var(key) {
            Ok(value) => {
                pulse_vars.insert(key.to_string(), value);
            }
            Err(_err) => {
                pulse_vars.insert(key.to_string(), "".to_string());
            }
        }
    }

    // get pulse client
    let pulse_client = pulse::client::new_client();

    // set auth headers
    let pulse_headers = pulse::client::new_headers(&pulse_vars["PULSE_TOKEN"]);

    // send query
    let chart_query: String;
    match pulse::api::get(
        &pulse_client,
        &pulse_vars["PULSE_ADDRESS"],
        &pulse_headers,
        "charts",
    ) {
        Ok(value) => {
            chart_query = value;
        }
        Err(error) => {
            println!("Error getting charts from pulse {}", error);
            chart_query = "".to_string();
        }
    }

    //let charts: pulse::types::Chart;
    let charts = match pulse::serde::parse_charts(&chart_query) {
        Ok(value) => value,
        Err(error) => {
            println!("Error parsing body for charts: {}", error);
            return;
        }
    };

    // create client to insert pulse data into db
    let mut pg_client = pg::client::new(
        &pulse_vars["POSTGRES_HOST"],
        &pulse_vars["POSTGRES_USER"],
        &pulse_vars["POSTGRES_PASSWORD"],
        "pulse",
    )
    .unwrap();

    // iterate through and push charts into db, do this instead of batch for now maybe refactor later
    for chart in charts.charts {
        pg::pulse::insert_chart(&mut pg_client, &chart.0, &chart.1);
    }
}
