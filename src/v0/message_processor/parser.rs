use super::message::{DataPayload, OTAPayload, PayloadType, Topic};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid topic format: {0}")]
    InvalidFormat(String),
    #[error("Missing topic part: {0}")]
    MissingPart(String),
}

#[derive(Debug)]
pub enum ChannelType {
    Data,
    Ota,
}

impl ChannelType {
    pub fn check_channel(channel: &str) -> Result<Self, ParseError> {
        match channel {
            "0" => Ok(ChannelType::Ota),
            "1" => Ok(ChannelType::Data),
            _ => Err(ParseError::InvalidFormat(
                "Invalid channel format".to_string(),
            )),
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn parse_topic(&self, topic: &str) -> Result<Topic, ParseError> {
        let mut parts = topic.split("/");

        let device_type = parts
            .next()
            .ok_or(ParseError::MissingPart("device_type".to_string()))?;
        let mac_id = parts
            .next()
            .ok_or(ParseError::MissingPart("mac_id".to_string()))?;
        let channel = parts
            .next()
            .ok_or(ParseError::MissingPart("channel".to_string()))?;

        if parts.next().is_some() {
            return Err(ParseError::InvalidFormat(topic.to_string()));
        }

        Ok(Topic {
            device_type: device_type.to_string(),
            mac_id: mac_id.to_string(),
            channel: channel.to_string(),
        })
    }

    pub fn parse_payload(
        &self,
        channel: &str,
        raw_payload: bytes::Bytes,
    ) -> Result<Box<dyn PayloadType>, ParseError> {
        fn from_slice<'a, T: Deserialize<'a>>(payload: &'a [u8]) -> Result<T, ParseError> {
            serde_json::from_slice(payload)
                .map_err(|e| ParseError::InvalidFormat(format!("Invalid payload format: {}", e)))
        }

        let v8_payload = &raw_payload.as_ref();

        match ChannelType::check_channel(channel)? {
            ChannelType::Data => {
                let data_payload: DataPayload = from_slice(v8_payload)?;
                Ok(Box::new(data_payload))
            }
            ChannelType::Ota => {
                let ota_payload: OTAPayload = from_slice(v8_payload)?;
                Ok(Box::new(ota_payload))
            }
        }
    }
}
