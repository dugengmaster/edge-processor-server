use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataChannelPayloadParser {
    slave_id: u8,
    device_info: String,
    timestamp: u64,
    data: HashMap<String, Value>,
}
