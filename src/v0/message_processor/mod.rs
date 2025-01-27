pub mod message;
mod parser;
mod validator;

use message::{
    PayloadType, Message, RawMessage, Topic,
};
use parser::{ParseError, Parser};
use validator::Validator;

pub struct MessageProcessor;

impl MessageProcessor {
    fn topic_processor(raw_message_topic: String) -> Result<Topic, ParseError> {
        Parser::parse_topic(raw_message_topic.clone()).and_then(|topic| {
            if Validator::validate_register_device(&topic) {
                Ok(topic)
            } else {
                Err(ParseError::InvalidFormat(
                    ("Invalid topic format: ".to_string() + &raw_message_topic).to_string(),
                ))
            }
        })
    }

    fn payload_processor<'a>(
        channel: &str,
        raw_message_payload: bytes::Bytes,
    ) -> Result<Box<dyn PayloadType + 'a>, ParseError> {
        Parser::parse_payload(channel, raw_message_payload)
    }

    pub fn message_processor<'a>(raw_message: RawMessage) -> Message<dyn PayloadType + 'a> {
        println!("message_processor");
        let topic = Self::topic_processor(raw_message.topic).unwrap();
        let payload = Self::payload_processor(&topic.channel, raw_message.payload).unwrap();
        Message::new(topic, payload) as Message<dyn PayloadType + 'a>
    }
}
