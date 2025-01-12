pub mod data_channel_payload_parser;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use data_channel_payload_parser::DataChannelPayloadParser;

pub trait PayloadParserTrait {
    fn parse(&self, payload: Vec<u8>) -> Result<ChannelPayload>;
}

struct PayloadParser {}
