pub struct Topic<'a> {
    pub device_type: &'a str,
    pub mac_id: &'a str,
    pub channel: &'a str,
}

pub struct TopicParser<'a> {
    topic: Option<Topic<'a>>,
}

impl<'a> TopicParser<'a> {
    pub fn new() -> Self {
        TopicParser { topic: None }
    }
    pub fn parse(topic: &'a str) -> Option<Self> {
        let mut parts = topic.split('/');

        let device_type: &str = parts.next()?;
        let mac_id: &str = parts.next()?;
        let channel: &str = parts.next()?;

        Some(TopicParser {
            topic: Some(Topic {
                device_type,
                mac_id,
                channel,
            }),
        })
    }

    pub fn get_topic(&self) -> Option<&Topic<'a>> {
        self.topic.as_ref()
    }
}
