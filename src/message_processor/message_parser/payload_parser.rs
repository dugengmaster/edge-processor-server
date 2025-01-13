pub mod channel_payload;

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::collections::HashMap;

use channel_payload::ChannelPayload;
// pub trait PayloadParserTrait {
//     fn parse(&self, payload: Vec<u8>) -> Result<ChannelPayload>;
// }

pub struct PayloadParser {}

impl PayloadParser {
    pub fn new(payload: &[u8]) -> Option<Self> {
        Some(PayloadParser {})
    }

    pub fn parse(&self, payload: &[u8]) -> Result<ChannelPayload> {
        let payload: ChannelPayload = serde_json::from_slice(payload)?;
        Ok(payload)
    }
}