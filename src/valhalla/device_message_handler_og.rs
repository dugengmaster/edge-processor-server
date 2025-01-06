// use crate::channel_config::{load_channel_configs, TopicInfo};
// use std::{collections::HashMap, sync::Arc};
// use std::str;
// use tokio::sync::mpsc::{self, Sender};

// #[derive(Debug)]
// pub struct DeviceMessage {
//     pub device_type: String,
//     pub mac_id: String,
//     pub payload: Vec<u8>,
// }

// pub struct DeviceMessageHandler {
//     senders: HashMap<String, Sender<DeviceMessage>>,
//     handlers: Arc<Vec<tokio::task::JoinHandle<()>>>,
// }

// impl Clone for DeviceMessageHandler {
//     fn clone(&self) -> Self {
//         let mut senders = HashMap::new();
//         for (key, sender) in &self.senders {
//             senders.insert(key.clone(), sender.clone());
//         }

//         DeviceMessageHandler {
//             senders,
//             self.handlers,
//         }
//     }
// }
// impl DeviceMessageHandler {
//     pub fn new() -> Self {
//         let mut senders = HashMap::new();
//         let mut handlers = Vec::new();

//         // 從配置載入頻道設定
//         let channel_configs = load_channel_configs();

//         // 為每個配置的頻道創建處理器
//         for (channel_type, config) in &channel_configs {
//             let (sender, mut receiver) = mpsc::channel::<DeviceMessage>(100);
//             let channel_name = config.channel_name.clone();

//             senders.insert(channel_type.clone(), sender);

//             // 建立預設的訊息處理器
//             let handle = tokio::spawn(async move {
//                 while let Some(message) = receiver.recv().await {
//                     println!(
//                         "Message received on channel {} from device type {} (MAC: {})",
//                         channel_name, message.device_type, message.mac_id
//                     );
//                     Self::print_message_content(&message.payload, &channel_name);
//                 }
//             });
//             handlers.push(handle);
//         }

//         DeviceMessageHandler {
//             senders,
//             handlers,
//         }
//     }

//     // 驗證設備類型和MAC地址（這裡您可以實現具體的驗證邏輯）
//     // fn validate_device(&self, device_type: &str, mac_id: &str) -> bool {
//     //     // TODO: 實現設備驗證邏輯
//     //     true // 暫時總是返回 true
//     // }

//     // 分發消息到對應的佇列
//     pub async fn dispatch_message(&self, topic: &str, payload: &[u8]) {
//         let topic_info = match TopicInfo::parse(topic) {
//             Some(info) => info,
//             None => {
//                 println!("Invalid topic format: {}", topic);
//                 return;
//             }
//         };

//         // 驗證設備
//         // if !self.validate_device(&topic_info.device_type, &topic_info.mac_id) {
//         //     println!("Invalid device: type={}, mac={}", topic_info.device_type, topic_info.mac_id);
//         //     return;
//         // }

//         // 根據 channel 找到對應的處理器
//         if let Some(sender) = self.senders.get(&topic_info.channel) {
//             let message = DeviceMessage {
//                 device_type: topic_info.device_type,
//                 mac_id: topic_info.mac_id,
//                 payload: payload.to_vec(),
//             };

//             if let Err(e) = sender.send(message).await {
//                 println!(
//                     "Failed to send message to channel {}: {}",
//                     topic_info.channel, e
//                 );
//             }
//         } else {
//             println!("Unknown channel type: {}", topic_info.channel);
//             Self::print_message_content(payload, "Unknown");
//         }
//     }

//     fn print_message_content(payload: &[u8], prefix: &str) {
//         match String::from_utf8(payload.to_vec()) {
//             Ok(message) => println!("{}: {}", prefix, message),
//             Err(_) => println!(
//                 "Binary {}, length: {} bytes",
//                 prefix.to_lowercase(),
//                 payload.len()
//             ),
//         }
//     }
// }
