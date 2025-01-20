// use crate::device_message_controller::DeviceMessageController;
// use crate::message_processor::message_parser::MessageParser;
// mod message_processor;

use crate::{RawMessage, message_processor::message_processor};

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(&self, raw_message: RawMessage) {
        message_processor(raw_message);
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