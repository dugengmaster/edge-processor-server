use queues::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 定義 Data 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataPayload {
    pub data: HashMap<String, f64>,
    // 設備資訊
    pub device_info: DeviceInfo,
    pub timestamp: String,
}

// 定義 DeviceInfo 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub device_type: String,
    pub brand: String,
    pub model: String,
    pub hp: String,
}

// 定義 Status 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusPayload {
    pub status: String,
}

// 定義 Configuration 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigurationPayload {
    pub config: String,
}

// 定義 Logger 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoggerPayload {
    pub message: String,
    pub level: String,
}

// 定義 Command 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandPayload {
    pub command: String,
    pub timestamp: String,
}

// 定義 OTA 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OTAPayload {
    pub version: String,
    pub url: String,
    pub update_type: String,
}

// 定義 HonneyPot 頻道 Payload 的結構
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HonneyPotPayload {
    pub data: String,
    pub timestamp: String,
}

// 定義頻道類型枚舉
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ChannelType {
    Status,
    Data,
    Configuration,
    Logger,
    Command,
    OTA,
    HonneyPot,
    Unknown,
}

// 定義頻道結構 (使用泛型)
pub struct Channel<T: Clone> {
    pub channel_type: ChannelType,
    pub queue: Buffer<T>,
}

impl<T: Clone> Channel<T> {
    // 建立新的頻道實例
    pub fn new(channel_type: ChannelType, capacity: usize) -> Self {
        Channel {
            channel_type,
            queue: Buffer::new(capacity),
        }
    }

    // 發送訊息到頻道佇列
    pub fn send(&mut self, payload: T) {
        self.queue.add(payload).ok();
    }

    // 從頻道佇列接收訊息
    pub fn receive(&mut self) -> Result<T, &str> {
        self.queue.remove()
    }
}

use std::sync::Mutex;

// 定義頻道管理器結構 (使用泛型)
pub struct ChannelManager {
    channels: Mutex<HashMap<ChannelType, Box<dyn std::any::Any + Send>>>,
}

impl ChannelManager {
    // 建立新的頻道管理器實例
    pub fn new() -> Self {
        ChannelManager {
            channels: Mutex::new(HashMap::new()),
        }
    }

    // 建立新的頻道
    pub fn create_channel<T: 'static + Send + Clone>(&self, channel_type: ChannelType, capacity: usize) {
        let mut channels = self.channels.lock().unwrap();
        let channel = Channel::<T>::new(channel_type, capacity);
        channels.insert(channel_type, Box::new(channel));
    }

    // 向指定頻道發送訊息
    pub fn send_message<T: 'static + Send + Clone>(
        &self,
        channel_type: ChannelType,
        payload: T,
    ) -> Result<(), String> {
        let mut channels = self.channels.lock().unwrap();
        if let Some(channel) = channels.get_mut(&channel_type) {
            if let Some(channel) = channel.downcast_mut::<Channel<T>>() {
                channel.send(payload);
                Ok(())
            } else {
                Err(format!("Channel {:?} has a different type", channel_type))
            }
        } else {
            Err(format!("Channel {:?} not found", channel_type))
        }
    }

    // 從指定頻道接收訊息
    pub fn receive_message<T: 'static + Send + Clone>(
        &self,
        channel_type: ChannelType,
    ) -> Result<Option<T>, String> {
        let mut channels = self.channels.lock().unwrap();
        if let Some(channel) = channels.get_mut(&channel_type) {
            if let Some(channel) = channel.downcast_mut::<Channel<T>>() {
                Ok(channel.receive().ok())
            } else {
                Err(format!("Channel {:?} has a different type", channel_type))
            }
        } else {
            Err(format!("Channel {:?} not found", channel_type))
        }
    }
}
