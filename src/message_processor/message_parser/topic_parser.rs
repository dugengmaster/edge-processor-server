use super::super::message::Topic;

pub struct TopicParser {
    topic: Option<Topic>,
}

impl<'a> TopicParser {
    pub fn new() -> Self {
        TopicParser { topic: None }
    }
    pub fn parse(&self, topic: &'a str) -> Option<Topic> {
        let mut parts = topic.split('/');

        let device_type= parts.next()?.to_string();
        let mac_id= parts.next()?.to_string();
        let channel= parts.next()?.to_string();

        Some(Topic {
            device_type,
            mac_id,
            channel,
        })
    }

    pub fn get_topic(&self) -> Option<Topic> {
        self.topic.clone()
    }
}
