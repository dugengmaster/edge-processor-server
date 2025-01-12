pub mod topic_parser;
pub mod payload_parser;

use topic_parser::TopicParser;
use payload_parser::PayloadParser;

pub struct MessageParser<'a> {
    topic_parser: TopicParser<'a>,
    payload_parser: PayloadParser,
}