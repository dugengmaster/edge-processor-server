use super::message::Topic;
use crate::device_model::DeviceModel;

pub struct Validator;

impl Validator {
    pub fn validate_register_device(topic: &Topic) -> bool {
        let is_device_register = match topic.device_type.as_str() {
            "DM" => DeviceModel::new().get_gateway_by_macid(&topic.mac_id).is_some(),
            _ => false,
        };
        
        if !is_device_register {
            eprintln!(
                "[ERROR] Unregistered device - Type: {}, MAC ID: {}",
                topic.device_type, topic.mac_id
            );
        }
        is_device_register
    }
}