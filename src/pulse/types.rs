use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// For deserializing and Serializing if I need to
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Charts {
    // remapping for convenience
    #[serde(rename = "data")]
    pub charts: HashMap<String, Chart>,
    #[serde(rename = "nodeData")]
    pub nodes: HashMap<String, Node>,
    #[serde(rename = "stats")]
    pub stats: Stat,
    #[serde(rename = "storageData")]
    pub storage: HashMap<String, Storage>,
    #[serde(rename = "timestamp")]
    pub time_stamp: i64,
}

/*
      originally did an enum for a dynamic type but this added a load of complications for
      the postgres use in the db and since we really only have 2 types so we'll do 2 for now
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct FloatValue {
    pub timestamp: i64,
    pub value: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NumValue {
    pub timestamp: i64,
    pub value: i64,
}

// Charts pushed by the api
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Chart {
    pub cpu: Vec<FloatValue>,
    pub disk: Vec<FloatValue>,
    pub diskread: Vec<NumValue>,
    pub diskwrite: Vec<NumValue>,
    pub memory: Vec<FloatValue>,
    pub netin: Vec<NumValue>,
    pub netout: Vec<NumValue>,
}

// nodes pushed by the api
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Node {
    pub cpu: Vec<FloatValue>,
    pub disk: Vec<FloatValue>,
    pub memory: Vec<FloatValue>,
}

// single stat pushed by the api
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Stat {
    #[serde(rename = "oldestDataTimestamp")]
    pub oldestdatatimestamp: i64,
}

// storage pushed by the api
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Storage {
    pub disk: Vec<FloatValue>,
}
