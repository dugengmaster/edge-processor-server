// use crate::v0::message_processor::message::{Message, PayloadType};

use super::message_processor::{message::RawMessage, MessageProcessor};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(
        &self,
        raw_message: RawMessage,
        message_processor: Arc<MessageProcessor>,
    ) {
        let start = Instant::now();
        let message = message_processor.message_processor(raw_message);

        println!(
            "[INFO] Message received - Type: {}, MAC: {}, Channel: {}, payload: {:?}",
            message.topic.device_type, message.topic.mac_id, message.topic.channel, message.payload
        );
        let duration = start.elapsed(); // 計算經過時間
        println!("[INFO] Message processing time: {:?}", duration); // 輸出處理時間
    }
}
