// 模擬資料庫結構
#[allow(dead_code)]
#[derive(Debug)]
pub struct Gateway {
    pub gateway_id: u32,
    pub gateway_macid: String,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Brand {
    pub brand_id: u32,
    pub brand_name: String,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct DeviceType {
    pub device_type_id: u32,
    pub device_type_name: String,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Model {
    pub model_id: u32,
    pub model_name: String,
    pub brand_id: u32,
    pub device_type_id: u32,
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct DeviceInfo {
    pub index: u32,
    pub gateway_id: u32,
    pub slaveid: u8,
    pub model_id: u32,
}
#[allow(dead_code)]
pub struct MockDatabase {
    pub gateways: Vec<Gateway>,
    pub brands: Vec<Brand>,
    pub device_types: Vec<DeviceType>,
    pub models: Vec<Model>,
    pub data: Vec<DeviceInfo>,
}
#[allow(dead_code)]
impl MockDatabase {
    pub fn new() -> Self {
        let gateways = vec![
            Gateway {
                gateway_id: 1,
                gateway_macid: "8CAAB5537EF2".to_string(),
            },
            Gateway {
                gateway_id: 2,
                gateway_macid: "C8C9A3583898".to_string(),
            },
            Gateway {
                gateway_id: 3,
                gateway_macid: "483FDA578E98".to_string(),
            },
            Gateway {
                gateway_id: 4,
                gateway_macid: "A020A624F3E0".to_string(),
            },
            Gateway {
                gateway_id: 5,
                gateway_macid: "BCFF4DCFDCEA".to_string(),
            },
            Gateway {
                gateway_id: 6,
                gateway_macid: "D8BFC0192998".to_string(),
            },
            Gateway {
                gateway_id: 7,
                gateway_macid: "483FDA44DFAF".to_string(),
            },
            Gateway {
                gateway_id: 8,
                gateway_macid: "BCDDC2568AA4".to_string(),
            },
        ];

        let brands = vec![
            Brand {
                brand_id: 1,
                brand_name: "HITACHI".to_string(),
            },
            Brand {
                brand_id: 2,
                brand_name: "COMATE".to_string(),
            },
            Brand {
                brand_id: 3,
                brand_name: "JAGUAR".to_string(),
            },
            Brand {
                brand_id: 4,
                brand_name: "PARKER".to_string(),
            },
        ];

        let device_types = vec![
            DeviceType {
                device_type_id: 1,
                device_type_name: "AirCompressor".to_string(),
            },
            DeviceType {
                device_type_id: 2,
                device_type_name: "MassFlowMeter".to_string(),
            },
            DeviceType {
                device_type_id: 3,
                device_type_name: "CompressedAirDryers".to_string(),
            },
        ];

        let models = vec![
            Model {
                model_id: 1,
                model_name: "OSP-37M6AN2".to_string(),
                brand_id: 1,
                device_type_id: 1,
            },
            Model {
                model_id: 2,
                model_name: "OSP-37VAG1".to_string(),
                brand_id: 1,
                device_type_id: 1,
            },
            Model {
                model_id: 3,
                model_name: "TGF200".to_string(),
                brand_id: 2,
                device_type_id: 2,
            },
            Model {
                model_id: 4,
                model_name: "ZLS-20Hi+3".to_string(),
                brand_id: 3,
                device_type_id: 1,
            },
            Model {
                model_id: 5,
                model_name: "LM-YED".to_string(),
                brand_id: 4,
                device_type_id: 3,
            },
        ];

        let data = vec![
            DeviceInfo {
                index: 1,
                gateway_id: 1,
                slaveid: 1,
                model_id: 1,
            },
            DeviceInfo {
                index: 2,
                gateway_id: 2,
                slaveid: 1,
                model_id: 2,
            },
            DeviceInfo {
                index: 3,
                gateway_id: 3,
                slaveid: 5,
                model_id: 3,
            },
            DeviceInfo {
                index: 4,
                gateway_id: 3,
                slaveid: 1,
                model_id: 4,
            },
            DeviceInfo {
                index: 5,
                gateway_id: 5,
                slaveid: 1,
                model_id: 5,
            },
            DeviceInfo {
                index: 6,
                gateway_id: 4,
                slaveid: 1,
                model_id: 1,
            },
        ];

        MockDatabase {
            gateways,
            brands,
            device_types,
            models,
            data,
        }
    }

    pub fn get_device_info(
        &self,
        gateway_macid: &str,
        slaveid: u8,
        brand_name: &str,
        device_type_name: &str,
        model_name: &str,
    ) -> bool {
        // Find gateway_id
        let mut gateway_id: Option<u32> = None;
        for gateway in &self.gateways {
            if gateway.gateway_macid == gateway_macid {
                gateway_id = Some(gateway.gateway_id);
                break;
            }
        }

        // Find brand_id
        let mut brand_id: Option<u32> = None;
        for brand in &self.brands {
            if brand.brand_name == brand_name {
                brand_id = Some(brand.brand_id);
                break;
            }
        }

        // Find device_type_id
        let mut device_type_id: Option<u32> = None;
        for device_type in &self.device_types {
            if device_type.device_type_name == device_type_name {
                device_type_id = Some(device_type.device_type_id);
                break;
            }
        }

        // Find model_id
        let mut model_id: Option<u32> = None;
        if let (Some(b_id), Some(dt_id)) = (brand_id, device_type_id) {
            for model in &self.models {
                if model.model_name == model_name
                    && model.brand_id == b_id
                    && model.device_type_id == dt_id
                {
                    model_id = Some(model.model_id);
                    break;
                }
            }
        }

        // Return false if any ID is not found
        if gateway_id.is_none() || model_id.is_none() {
            return false;
        }

        // Check if device exists
        let gateway_id = gateway_id.unwrap();
        let model_id = model_id.unwrap();

        for device in &self.data {
            if device.gateway_id == gateway_id
                && device.slaveid == slaveid
                && device.model_id == model_id
            {
                return true;
            }
        }
        false
    }
}
