use super::message::Topic;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid topic format: {0}")]
    InvalidFormat(String),
    #[error("Missing topic part: {0}")]
    MissingPart(String),
}

pub struct Parser;

impl Parser {
    pub fn parse_topic(topic: String) -> Result<Topic, ParseError> {
        let mut parts = topic.split("/");

        let device_type = parts
            .next()
            .ok_or(ParseError::MissingPart("device_type".to_string()))?
            .to_string();
        let mac_id = parts
            .next()
            .ok_or(ParseError::MissingPart("mac_id".to_string()))?
            .to_string();
        let channel = parts
            .next()
            .ok_or(ParseError::MissingPart("channel".to_string()))?
            .to_string();

        if parts.next().is_some() {
            return Err(ParseError::InvalidFormat(topic));
        }

        Ok(Topic {
            device_type,
            mac_id,
            channel,
        })
    }
}
