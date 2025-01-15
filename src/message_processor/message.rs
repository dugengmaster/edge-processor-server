use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct Message<C> where C: Channel {
    pub topic: Option<Topic>,
    pub payload: Option<C>,
}

impl<C: Channel> Message<C>{
    pub fn new() -> Self {
        Message {
            topic: None,
            payload: None,
        }
    }
    pub fn set_topic(mut self, topic: Topic) {
        self.topic = Some(topic);
    }
    pub fn set_payload(mut self, payload: C) {
        self.payload = Some(payload);
    }

    fn message_channel(&self) -> Option<&str> {
        self.topic.as_ref().map(|t| t.channel.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub device_type: String,
    pub mac_id: String,
    pub channel: String,
}

pub trait Channel {
    fn target_channel(&self) -> &'static str;
}

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

impl Channel for DataPayload {
    fn target_channel(&self) -> &'static str {
        "1"
    }
}

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

impl Channel for OTAPayload {
    fn target_channel(&self) -> &'static str {
        "0"
    }
}

type MqttMessage = crate::message_processor::message::Message<dyn Channel>;
