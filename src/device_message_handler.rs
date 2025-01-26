// use crate::device_message_controller::DeviceMessageController;
// use crate::message_processor::MessageProcessor;
mod message_processor;

use crate::{RawMessage, message_processor::MessageProcessor};

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(&self, raw_message: RawMessage) {
        let message = MessageProcessor::message_processor(raw_message);
        println!("[INFO] Message received - Type: {}, MAC: {}, Channel: {}", message.topic.device_type, message.topic.mac_id, message.topic.channel);
        
        // let topic = raw_message.topic;
        // let payload = raw_message.payload;

        // parser = MessageParser

        // if let Some(message) = MessageParser::new(topic, payload) {
        //     println!(
        //         "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
        //         message.device_type, message.mac_id, message.channel
        //     );
        // }
    }
}