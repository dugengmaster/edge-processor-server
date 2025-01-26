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

pub struct Message<C>
where
    C: Channel + ?Sized,
{
    pub topic: Topic,
    pub payload: Box<C>,
}

impl<C: Channel + ?Sized> Message<C> {
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

pub trait Channel: Debug {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: String,
    pub brand: String,
    pub model: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAInfo {
    pub version: String,
    pub url: String,
    pub update_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasePayload {
    pub slave_id: u8,
    pub device_info: DeviceInfo,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfiguration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPayload {
    pub base: BasePayload,
    pub data: HashMap<String, Value>,
}

impl Channel for DataPayload {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPayloadNow {
    pub slaveID: u8,
    pub model: String,
    pub timestamp: String,
    pub data: HashMap<String, Value>,
}

impl Channel for DataPayloadNow {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusPayload {
    pub base: BasePayload,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandPayload {
    pub base: BasePayload,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationPayload {
    pub base: BasePayload,
    pub config: DeviceConfiguration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerPayload {
    pub base: BasePayload,
    pub message: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAPayload {
    pub base: BasePayload,
    pub ota_info: OTAInfo,
}

impl Channel for OTAPayload {}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAPayloadNow {
    pub base: BasePayload,
    pub ota_info: OTAInfo,
}

impl Channel for OTAPayloadNow {}
