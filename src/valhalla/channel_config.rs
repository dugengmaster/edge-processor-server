use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TopicInfo {
    pub device_type: String,
    pub mac_id: String,
    pub channel: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelConfig {
    pub channel_name: String,    // 頻道名稱，用於內部識別
    pub channel_type: String,    // 對應到 topic 中的 channel 值
    pub description: String,
}

pub fn load_channel_configs() -> HashMap<String, ChannelConfig> {
    // 在實際應用中，這裡可以從檔案或資料庫載入
    let configs = vec![
        ChannelConfig {
            channel_name: "data".to_string(),
            channel_type: "1".to_string(),
            description: "Device data channel".to_string(),
        },
        ChannelConfig {
            channel_name: "status".to_string(),
            channel_type: "0".to_string(),
            description: "Device status channel".to_string(),
        },
    ];

    configs.into_iter().map(|config| (config.channel_type.clone(), config)).collect()
}

impl TopicInfo {
    pub fn parse(topic: &str) -> Option<Self> {
        let parts: Vec<&str> = topic.split('/').collect();
        if parts.len() != 3 {
            return None;
        }

        Some(TopicInfo {
            device_type: parts[0].to_string(),
            mac_id: parts[1].to_string(),
            channel: parts[2].to_string(),
        })
    }
}