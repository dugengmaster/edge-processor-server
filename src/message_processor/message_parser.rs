pub mod payload_parser;
pub mod topic_parser;

use payload_parser::PayloadParser;
use topic_parser::TopicParser;

pub struct MqttMessage {
    topic: String,
    payload: Vec<u8>,
}

pub struct MessageParser<'a> {
    pub topic_parser: Option<&'a TopicParser<'a>>,
    pub payload_parser: Option<&'a PayloadParser>,
}

impl<'a> MessageParser<'a> {
    pub fn new() -> Self {
        MessageParser {
            topic_parser: None,
            payload_parser: None,
        }
    }

    pub fn set_topic_parser(&mut self, topic_parser: &'a TopicParser<'a>) {
        self.topic_parser = Some(topic_parser);
    }

     pub fn set_payload_parser(&mut self, payload_parser: &'a PayloadParser) {
        self.payload_parser = Some(payload_parser);
    }


    pub fn parse_topic(&self, topic: &'a str) -> Option<TopicParser<'a>> {
        TopicParser::new(topic)
    }

    // pub fn parse_payload(&self, payload_data: &str, device_info: DeviceInfo) -> Option<ChannelPayload> {
    //        match self.topic_parser{
    //             Some(topic_parser) => PayloadParser::parse(topic_parser, payload_data, device_info),
    //             None => None,
    //         }
    // }
}
