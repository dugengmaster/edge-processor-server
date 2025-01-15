pub mod payload_parser;
pub mod topic_parser;

// use payload_parser::PayloadParser;
use topic_parser::TopicParser;
use super::message::Topic;

pub struct MessageParser {
    pub topic_parser: Option<TopicParser>,
    // pub payload_parser: Option<PayloadParser>,
}

impl MessageParser {
    pub fn new() -> Self {
        MessageParser {
            topic_parser: None,
            // payload_parser: None,
        }
    }

    // pub fn set_topic_parser(&mut self, topic_parser: &TopicParser) {
    //     self.topic_parser = Some(topic_parser);
    // }

    //  pub fn set_payload_parser(&mut self, payload_parser: &PayloadParser) {
    //     self.payload_parser = Some(payload_parser);
    // }


    fn parse_topic(&self, topic: &str) -> Option<Topic> {
        let parser = TopicParser::new();
        parser.parse(topic);
        parser.get_topic()
    }

    // pub fn parse_payload(&self, payload_data: &str, device_info: DeviceInfo) -> Option<ChannelPayload> {
    //        match self.topic_parser{
    //             Some(topic_parser) => PayloadParser::parse(topic_parser, payload_data, device_info),
    //             None => None,
    //         }
    // }
}
