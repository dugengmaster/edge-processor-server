use bytes::Bytes;
use rumqttc::Publish;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct RawMessage {
    pub topic: String,
    pub payload: Bytes,
}

// Implements a conversion from rumqttc's Publish to RawMessage.
impl From<Publish> for RawMessage {
    fn from(publish: Publish) -> Self {
        RawMessage {
            topic: publish.topic,
            payload: publish.payload,
        }
    }
}

#[derive(Debug)]
pub struct Message<C>
where
    C: PayloadType + ?Sized,
{
    pub topic: Topic,
    pub payload: Box<C>,
}

impl<C: PayloadType + ?Sized> Message<C> {
    pub fn new(topic: Topic, payload: Box<C>) -> Self {
        Message { topic, payload }
    }
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub device_type: String,
    pub mac_id: String,
    pub channel: String,
}

pub trait PayloadType: Debug {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPayload {
    pub slaveID: u8,
    pub model: String,
    pub timestamp: String,
    pub data: HashMap<String, Value>,
}

impl PayloadType for DataPayload {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAPayload {
    pub version: String,
    pub url: String,
    pub update_type: String,
}

impl PayloadType for OTAPayload {}

