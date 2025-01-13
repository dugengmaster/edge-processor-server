use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub struct BaseChannelPayload {
    pub slave_id: u8,
    pub device_info: DeviceInfo,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfiguration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataChannelPayload {
    pub base: BaseChannelPayload,
    data: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusChannelPayload {
    pub base: BaseChannelPayload,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandChannelPayload {
    pub base: BaseChannelPayload,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationChannelPayload {
    pub base: BaseChannelPayload,
    pub config: DeviceConfiguration,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerChannelPayload {
    pub base: BaseChannelPayload,
    pub message: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAPayload {
    pub base: BaseChannelPayload,
    pub ota_info: OTAInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChannelPayload {
    Data(DataChannelPayload),
    Status(StatusChannelPayload),
    Command(CommandChannelPayload),
    Configuration(ConfigurationChannelPayload),
    Logger(LoggerChannelPayload),
    OTA(OTAPayload),
}