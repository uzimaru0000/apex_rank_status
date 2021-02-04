use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Model {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub segments: Vec<Segment>,
}

#[derive(Debug, Deserialize)]
pub struct Segment {
    pub stats: HashMap<String, Stat>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    pub display_category: String,
    pub display_name: String,
    pub display_type: String,
    pub display_value: String,
    pub percentile: Option<f32>,
    pub rank: Option<u32>,
    pub value: f32,
}
