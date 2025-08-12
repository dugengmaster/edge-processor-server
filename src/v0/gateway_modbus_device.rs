use serde::{Deserialize, Serialize};
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Gateway {
    pub gateway_id: u32,
    pub gateway_macid: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Brand {
    pub brand_id: u32,
    pub brand_name: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DeviceType {
    pub device_type_id: u32,
    pub device_type_name: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Model {
    pub model_id: u32,
    pub model_name: String,
    pub brand_id: u32,
    pub device_type_id: u32,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DeviceInfo {
    pub index: u32,
    pub gateway_id: u32,
    pub slaveid: u8,
    pub model_id: u32,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DataMapping {
    pub data_key: String,
    pub chinese_description: String,
    pub eng_description: String, // 新增英文描述欄位
    pub unit: String,            // 新增單位欄位
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MockDatabase {
    pub gateways: Vec<Gateway>,
    pub brands: Vec<Brand>,
    pub device_types: Vec<DeviceType>,
    pub models: Vec<Model>,
    pub data: Vec<DeviceInfo>,
    pub data_mapping: Vec<DataMapping>,
}
#[allow(dead_code)]
impl MockDatabase {
    pub fn new() -> Self {
        if let Ok(json_content) = fs::read_to_string("database.json") {
            if let Ok(database) = serde_json::from_str::<MockDatabase>(&json_content) {
                return database;
            }
        }

        MockDatabase {
            gateways: vec![],
            brands: vec![],
            device_types: vec![],
            models: vec![],
            data: vec![],
            data_mapping: vec![],
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
