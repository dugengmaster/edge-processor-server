use std::collections::HashMap;

// device_list for testing
#[derive(Debug, Clone)]
pub struct DeviceList {
    // Key: device type
    // Value: list of MAC IDs
    devices: HashMap<String, Vec<String>>,
}

impl DeviceList {
    pub fn new() -> Self {
        let mut devices = HashMap::new();

        // 模擬資料庫中 DM 類型設備的 MAC ID 列表
        devices.insert(
            "DM".to_string(),
            vec![
                "BCDDC2568AA4".to_string(),
                "BCFF4DCFDCEA".to_string(),
                "483FDA44DFAF".to_string(),
                "483FDA578E98".to_string(),
                "BCDD230381F".to_string(),
                "C8C9A3583898".to_string(),
                "BCFF4DCFDCEA".to_string(),
                "8CAAB5537EF2".to_string(),
                "A020A624F3E0".to_string(),
            ],
        );

        DeviceList { devices }
    }

    pub fn is_valid_device(&self, device_type: &str, mac_id: &str) -> bool {
        if let Some(allowed_macs) = self.devices.get(&device_type.to_uppercase()) {
            allowed_macs.contains(&mac_id.to_uppercase())
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceMessage {
    pub device_type: String,
    pub mac_id: String,
    pub channel: String,
    pub payload: Vec<u8>,
}

impl DeviceMessage {
    pub fn new(topic: &str, payload: &Vec<u8>) -> Option<Self> {
        Self::parse(topic, payload)
    }

    fn validate_device(&self) -> bool {
        let device_list = DeviceList::new();
        let is_valid = device_list.is_valid_device(&self.device_type, &self.mac_id);
        if !is_valid {
            eprintln!(
                "[ERROR] Unregistered device - Type: {}, MAC ID: {}",
                self.device_type, self.mac_id
            );
        }
        is_valid
    }

    fn parse(topic: &str, payload: &Vec<u8>) -> Option<Self> {
        let parts: Vec<&str> = topic.split('/').collect();
        if parts.len() != 3 {
            eprintln!("[ERROR] Invalid topic format: {}", topic);
            return None;
        }

        let parser = DeviceMessage {
            device_type: parts[0].to_string().to_uppercase(),
            mac_id: parts[1].to_string().to_uppercase(),
            channel: parts[2].to_string(),
            payload: payload.clone(),
        };

        if !parser.validate_device() {
            return None;
        }

        Some(parser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_topic() {
        let topic = "DM/BCDDC2568AA4/data";
        let payload = vec![0x01, 0x02, 0x03];
        let parser = DeviceMessage::parse(topic, &payload);
        assert!(parser.is_some(), "應該能成功解析有效的主題");
        let parser = parser.unwrap();
        assert_eq!(parser.device_type, "DM");
        assert_eq!(parser.mac_id, "BCDDC2568AA4");
        assert_eq!(parser.channel, "data");
    }

    #[test]
    fn test_parse_invalid_topic() {
        // 測試格式不正確的主題
        let topic = "invalid/topic";
        let payload = vec![];
        let parser = DeviceMessage::parse(topic, &payload);
        assert!(parser.is_none(), "格式不正確的主題應該返回 None");

        // 測試無效的設備類型
        let topic = "INVALID/BCDDC2568AA4/data";
        let payload = vec![0x01, 0x02, 0x03];
        let parser = DeviceMessage::parse(topic, &payload);
        assert!(parser.is_none(), "無效的設備類型應該返回 None");

        // 測試無效的 MAC ID
        let topic = "DM/INVALID_MAC/data";
        let payload = vec![0x01, 0x02, 0x03];
        let parser = DeviceMessage::parse(topic, &payload);
        assert!(parser.is_none(), "無效的 MAC ID 應該返回 None");
    }

    #[test]
    fn test_device_validation() {
        let device_list = DeviceList::new();

        // 測試有效的組合
        assert!(
            device_list.is_valid_device("DM", "BCDDC2568AA4"),
            "應該驗證通過有效的設備類型和MAC組合"
        );

        // 測試無效的MAC ID
        assert!(
            !device_list.is_valid_device("DM", "INVALID_MAC"),
            "應該拒絕無效的MAC ID"
        );

        // 測試無效的設備類型
        assert!(
            !device_list.is_valid_device("INVALID", "BCDDC2568AA4"),
            "應該拒絕無效的設備類型"
        );
    }
}
