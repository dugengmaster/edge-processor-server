pub mod message;
pub mod parser;
pub mod validator;

use serde_json::json;
use std::collections::HashMap;

use message::{BasePayload, DataPayload, DeviceInfo, Message, RawMessage, Topic};
pub use parser::Parser;
// pub use validator::Validator;

fn generate_test_message() -> Message<DataPayload> {
    let device_info = DeviceInfo {
        device_type: "sensor".to_string(),
        brand: "Acme".to_string(),
        model: "ModelX".to_string(),
    };

    let base_payload = BasePayload {
        slave_id: 1,
        device_info,
        timestamp: 1678886400,
    };

    let mut data = HashMap::new();
    data.insert("temperature".to_string(), json!(25.5));
    data.insert("humidity".to_string(), json!(60.2));

    let topic = Topic {
        device_type: "sensor".to_string(),
        mac_id: "00:11:22:33:44:55".to_string(),
        channel: "data".to_string(),
    };

    let data_payload = DataPayload {
        base: base_payload,
        data,
    };

    Message::new().set_topic(topic).set_payload(data_payload)
}

pub fn message_processor(raw_message: RawMessage) -> Message<DataPayload> {
    println!("message_processor");
    if let topic = Parser::parse_topic(raw_message.topic) {}

    generate_test_message()
}
