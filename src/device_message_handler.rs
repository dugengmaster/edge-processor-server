use crate::device_message::DeviceMessage;
use rumqttc::Publish;
use std::str;

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(&self, publish: Publish) {
        let topic = publish.topic;
        let payload = publish.payload.to_vec();

        if let Some(message) = DeviceMessage::new(&topic, &payload) {
            println!(
                "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
                message.device_type, message.mac_id, message.channel
            );
            self.process_message(message).await;
        }
    }

    async fn process_message(&self, message: DeviceMessage) {
        match message.channel.as_str() {
            "0" => {
                match str::from_utf8(&message.payload) {
                    Ok(text) => println!("[INFO] Message content: {}", text),
                    Err(_) => eprintln!("[ERROR] Invalid UTF-8 sequence in payload"),
                }
            }
            "1" => {
                match str::from_utf8(&message.payload) {
                    Ok(text) => println!("[DATA] Message content: {}", text),
                    Err(_) => eprintln!("[ERROR] Invalid UTF-8 sequence in payload"),
                }
            }
            _ => {
                eprintln!(
                    "[ERROR] Unsupported message type - Type: {}, Channel: {}",
                    message.device_type, message.channel
                );
            }
        }
    }
}