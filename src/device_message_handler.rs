use std::str;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[derive(Debug)]
pub struct DeviceMessage {
    device_id: String,
    payload: Vec<u8>,
}

#[derive(Clone)]
pub struct DeviceMessageHandler {
    data_sender: Sender<DeviceMessage>,
    status_sender: Sender<DeviceMessage>,
}

impl DeviceMessageHandler {
    pub fn new() -> (Self, (Receiver<DeviceMessage>, Receiver<DeviceMessage>)) {
        let (data_sender, data_receiver) = mpsc::channel(100);
        let (status_sender, status_receiver) = mpsc::channel(100);

        (
            DeviceMessageHandler {
                data_sender,
                status_sender,
            },
            (data_receiver, status_receiver),
        )
    }

    // 分發消息到對應的佇列
    pub async fn dispatch_message(&self, topic: &str, payload: &[u8]) {
        let parts: Vec<&str> = topic.split('/').collect();

        if parts.len() < 3 {
            println!("Invalid topic format: {}", topic);
            return;
        }

        let message = DeviceMessage {
            device_id: parts[1].to_string(),
            payload: payload.to_vec(),
        };

        match (parts[0], parts[2]) {
            ("DM", "1") => {
                if let Err(e) = self.data_sender.send(message).await {
                    println!("Failed to send data message: {}", e);
                }
            }
            ("DM", "0") => {
                if let Err(e) = self.status_sender.send(message).await {
                    println!("Failed to send status message: {}", e);
                }
            }
            _ => {
                println!("Unknown message format:");
                println!("Topic: {}", topic);
                Self::print_message_content(payload, "Content");
            }
        }
    }
    
    // 處理數據消息
    pub async fn handle_data_message(message: DeviceMessage) {
        println!("Device Data Message from device: {}", message.device_id);
        Self::print_message_content(&message.payload, "Data");
        // 這裡可以添加具體的數據處理邏輯
    }

    // 處理狀態消息
    pub async fn handle_status_message(message: DeviceMessage) {
        println!("Device Status Message from device: {}", message.device_id);
        Self::print_message_content(&message.payload, "Status");
        // 這裡可以添加具體的狀態處理邏輯
    }

    fn print_message_content(payload: &[u8], prefix: &str) {
        match String::from_utf8(payload.to_vec()) {
            Ok(message) => println!("{}: {}", prefix, message),
            Err(_) => println!("Binary {}, length: {} bytes", prefix.to_lowercase(), payload.len()),
        }
    }
}