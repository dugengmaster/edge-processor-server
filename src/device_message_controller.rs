use crate::device_model::DeviceModel;


#[derive(Debug, Clone)]
pub struct DeviceMessageController {
    pub device_type: String,
    pub mac_id: String,
    pub channel: String,
    pub payload: Vec<u8>,
}

impl DeviceMessageController {
    pub fn new(topic: String, payload: Vec<u8>) -> Option<Self> {
        Self::parse(topic, payload)
    }

    pub fn validate_register_device(&self) -> bool {
        let is_device_register = match self.device_type.as_str() {
            "DM" => DeviceModel::new().get_gateway_by_macid(&self.mac_id).is_some(),
            _ => false,
        };
        
        if !is_device_register {
            eprintln!(
                "[ERROR] Unregistered device - Type: {}, MAC ID: {}",
                self.device_type, self.mac_id
            );
        }
        is_device_register
    }

    // pub fn validate_modbus_device_connection(&self) -> bool {
    //     let is_modbus_device_connected = match self.
    // }

    fn parse(topic: String, payload: Vec<u8>) -> Option<Self> {
        let parts: Vec<&str> = topic.split('/').collect();
        if parts.len() != 3 {
            eprintln!("[ERROR] Invalid topic format: {}", topic);
            return None;
        }

        let parser = DeviceMessageController {
            device_type: parts[0].to_string().to_uppercase(),
            mac_id: parts[1].to_string().to_uppercase(),
            channel: parts[2].to_string(),
            payload: payload.clone(),
        };

        // if !parser.validate_device() {
        //     return None;
        // }

        Some(parser)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_valid_topic() {
//         let topic = "DM/BCDDC2568AA4/data";
//         let payload = vec![0x01, 0x02, 0x03];
//         let parser = DeviceMessageController::parse(topic, payload);
//         assert!(parser.is_some(), "應該能成功解析有效的主題");
//         let parser = parser.unwrap();
//         assert_eq!(parser.device_type, "DM");
//         assert_eq!(parser.mac_id, "BCDDC2568AA4");
//         assert_eq!(parser.channel, "data");
//     }

//     #[test]
//     fn test_parse_invalid_topic() {
//         // 測試格式不正確的主題
//         let topic = "invalid/topic";
//         let payload = vec![];
//         let parser = DeviceMessageController::parse(topic, &payload);
//         assert!(parser.is_none(), "格式不正確的主題應該返回 None");

//         // 測試無效的設備類型
//         let topic = "INVALID/BCDDC2568AA4/data";
//         let payload = vec![0x01, 0x02, 0x03];
//         let parser = DeviceMessageController::parse(topic, &payload);
//         assert!(parser.is_none(), "無效的設備類型應該返回 None");

//         // 測試無效的 MAC ID
//         let topic = "DM/INVALID_MAC/data";
//         let payload = vec![0x01, 0x02, 0x03];
//         let parser = DeviceMessageController::parse(topic, &payload);
//         assert!(parser.is_none(), "無效的 MAC ID 應該返回 None");
//     }

    // #[test]
    // fn test_device_validation() {
    //     let device_list = RegisterDeviceList::new();

    //     // 測試有效的組合
    //     assert!(
    //         device_list.is_valid_device("DM", "BCDDC2568AA4"),
    //         "應該驗證通過有效的設備類型和MAC組合"
    //     );

    //     // 測試無效的MAC ID
    //     assert!(
    //         !device_list.is_valid_device("DM", "INVALID_MAC"),
    //         "應該拒絕無效的MAC ID"
    //     );

    //     // 測試無效的設備類型
    //     assert!(
    //         !device_list.is_valid_device("INVALID", "BCDDC2568AA4"),
    //         "應該拒絕無效的設備類型"
    //     );
    // }
// }
