use super::message_processor::{message::RawMessage, MessageProcessor};

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(&self, raw_message: RawMessage) {
        let message = MessageProcessor::message_processor(raw_message);
        println!("[INFO] Message received - Type: {}, MAC: {}, Channel: {}, payload: {:?}", message.topic.device_type, message.topic.mac_id, message.topic.channel, message.payload);
        
    }
}