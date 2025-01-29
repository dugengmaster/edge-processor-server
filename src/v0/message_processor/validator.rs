use super::message::Topic;
use crate::v0::device_model::DeviceModel;

pub struct Validator {
    device_model: DeviceModel,
}

impl Validator {
    pub fn new() -> Self {
        Validator {
            device_model: DeviceModel::new(),
        }
    }

    pub fn validate_register_device(&self, topic: &Topic) -> bool {
        let is_device_register = match topic.device_type.as_str() {
            "DM" => self.device_model.get_gateway_by_macid(&topic.mac_id).is_some(),
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
