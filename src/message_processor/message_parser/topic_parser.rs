pub struct TopicParser<'a> {
    pub device_type: &'a str,
    pub mac_id: &'a str,
    pub channel: &'a str,
}

impl<'a> TopicParser<'a> {
    pub fn new(topic: &'a str) -> Option<Self> {
        let mut parts = topic.split('/');
        
        let device_type: &str = parts.next()?;
        let mac_id: &str = parts.next()?;
        let channel: &str = parts.next()?;

        Some(TopicParser {
            device_type,
            mac_id,
            channel,
        })
    }
}