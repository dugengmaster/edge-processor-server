pub mod message;
mod parser;
mod validator;
use std::time::Instant;

use message::{
    PayloadType, Message, RawMessage, Topic,
};
use parser::{ParseError, Parser};
use validator::Validator;

pub struct MessageProcessor {
    parser: Parser,
    validator: Validator,
}


impl MessageProcessor {
    pub fn new() -> Self {
        MessageProcessor {
            parser: Parser,
            validator: Validator::new(),
        }
    }

    fn topic_processor(&self, raw_message_topic: &str) -> Result<Topic, ParseError> {
        self.parser.parse_topic(raw_message_topic).and_then(|topic| {
            if self.validator.validate_register_device(&topic) {
                Ok(topic)
            } else {
                Err(ParseError::InvalidFormat(format!(
                    "Invalid topic format: {}",
                    raw_message_topic
                )))
            }
        })
    }

    // fn topic_processor(&self, raw_message_topic: &str) -> Result<Topic, ParseError> {
    //     let parse_start_time = Instant::now();
    //     let topic_result = self.parser.parse_topic(raw_message_topic);
    //     let parse_time = parse_start_time.elapsed();
    //     eprintln!("parse_topic time: {:?}", parse_time);
    
    //     topic_result.and_then(move |topic| {
    //         let validate_start_time = Instant::now();
    //         if self.validator.validate_register_device(&topic) {
    //             let validate_time = validate_start_time.elapsed();
    //             eprintln!("validate_register_device time: {:?}", validate_time);
    //             Ok(topic)
    //         } else {
    //             let validate_time = validate_start_time.elapsed();
    //              eprintln!("validate_register_device time: {:?}", validate_time);
    //             Err(ParseError::InvalidFormat(format!(
    //                 "Invalid topic format: {}",
    //                 raw_message_topic
    //             )))
    //         }
    //     })
    // }

    fn payload_processor<'a>(
        &self, 
        channel: &str,
        raw_message_payload: bytes::Bytes,
    ) -> Result<Box<dyn PayloadType + 'a>, ParseError> {
        self.parser.parse_payload(channel, raw_message_payload)
    }

    pub fn message_processor<'a>(&self, raw_message: RawMessage) -> Message<dyn PayloadType + 'a> {
        let topic = self.topic_processor(&raw_message.topic).unwrap();
        let payload = self.payload_processor(&topic.channel, raw_message.payload).unwrap();
        Message::new(topic, payload) as Message<dyn PayloadType + 'a>
    }
    // pub fn message_processor<'a>(&self, raw_message: RawMessage) -> Message<dyn PayloadType + 'a> {
    //     let start_time = Instant::now();
    //     let topic_result = self.topic_processor(&raw_message.topic);
    //     let topic = match topic_result {
    //         Ok(t) => {
    //             eprintln!("topic_processor success: {:?}", t);
    //             t
    //         },
    //         Err(e) => {
    //             eprintln!("topic_processor error: {:?}", e);
    //             panic!("topic_processor failed");
    //         }
    //     };
    //     let topic_time = start_time.elapsed();
    //     eprintln!("topic_processor time: {:?}", topic_time);
    //     let start_time = Instant::now();
    //     let payload_result = self.payload_processor(&topic.channel, raw_message.payload);
    //         let payload = match payload_result {
    //         Ok(p) => {
    //             eprintln!("payload_processor success");
    //             p
    //         },
    //         Err(e) => {
    //             eprintln!("payload_processor error: {:?}", e);
    //             panic!("payload_processor failed");
    //         }
    //     };
    //     let payload_time = start_time.elapsed();
    //     eprintln!("payload_processor time: {:?}", payload_time);
    //     Message::new(topic, payload) as Message<dyn PayloadType + 'a>
    // }
}
