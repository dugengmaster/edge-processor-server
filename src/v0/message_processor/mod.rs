pub mod message;
pub mod parser;
mod validator;

use message::{Message, PayloadType, RawMessage, Topic};
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
        self.parser
            .parse_topic(raw_message_topic)
            .and_then(|topic| {
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

    fn payload_processor<'a>(
        &self,
        channel: &str,
        raw_message_payload: bytes::Bytes,
    ) -> Result<Box<dyn PayloadType + 'a>, ParseError> {
        self.parser.parse_payload(channel, raw_message_payload)
    }

    pub fn message_processor<'a>(
        &self,
        raw_message: RawMessage,
    ) -> Result<Message<dyn PayloadType + 'a>, ParseError> {
        let topic = match self.topic_processor(&raw_message.topic) {
            Ok(topic) => topic,
            Err(e) => return Err(e),
        };

        let payload = match self.payload_processor(&topic.channel, raw_message.payload) {
            Ok(payload) => payload,
            Err(e) => return Err(e),
        };

        Ok(Message::new(topic, payload))
    }
}
