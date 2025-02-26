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
#[derive(Debug)]
pub struct DataMapping {
    pub data_key: String,
    pub chinese_description: String,
    pub unit: String,  // 新增單位欄位
}
#[allow(dead_code)]
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
        let gateways = vec![
            // Gateway {
            //     gateway_id: 1,
            //     gateway_macid: "8CAAB5537EF2".to_string(),
            // },
            // Gateway {
            //     gateway_id: 2,
            //     gateway_macid: "C8C9A3583898".to_string(),
            // },
            Gateway {
                gateway_id: 3,
                gateway_macid: "483FDA578E98".to_string(),
            },
            // Gateway {
            //     gateway_id: 4,
            //     gateway_macid: "A020A624F3E0".to_string(),
            // },
            // Gateway {
            //     gateway_id: 5,
            //     gateway_macid: "BCFF4DCFDCEA".to_string(),
            // },
            // Gateway {
            //     gateway_id: 6,
            //     gateway_macid: "D8BFC0192998".to_string(),
            // },
            // Gateway {
            //     gateway_id: 7,
            //     gateway_macid: "483FDA44DFAF".to_string(),
            // },
            // Gateway {
            //     gateway_id: 8,
            //     gateway_macid: "BCDDC2568AA4".to_string(),
            // },
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

        let data_mapping = vec![
            DataMapping {
                data_key: "0".to_string(),
                chinese_description: "供氣壓力".to_string(),
                unit: "Mpa".to_string(),
            },
            DataMapping {
                data_key: "1".to_string(),
                chinese_description: "排氣溫度".to_string(),
                unit: "℃".to_string(),
            },
            DataMapping {
                data_key: "2".to_string(),
                chinese_description: "運行總時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "3".to_string(),
                chinese_description: "負載總時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "4".to_string(),
                chinese_description: "主機三相電流A".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "5".to_string(),
                chinese_description: "主機三相電流B".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "6".to_string(),
                chinese_description: "主機三相電流C".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "10".to_string(),
                chinese_description: "油濾器使用時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "11".to_string(),
                chinese_description: "油分器使用時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "12".to_string(),
                chinese_description: "空濾器使用時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "13".to_string(),
                chinese_description: "潤滑油使用時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "14".to_string(),
                chinese_description: "潤滑脂使用時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "17".to_string(),
                chinese_description: "風機三相電流A".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "18".to_string(),
                chinese_description: "風機三相電流B".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "19".to_string(),
                chinese_description: "風機三相電流C".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "22".to_string(),
                chinese_description: "相序保護電壓".to_string(),
                unit: "V".to_string(),
            },
            DataMapping {
                data_key: "23".to_string(),
                chinese_description: "缺相保護電壓".to_string(),
                unit: "V".to_string(),
            },
            DataMapping {
                data_key: "24".to_string(),
                chinese_description: "電源電壓".to_string(),
                unit: "V".to_string(),
            },
            DataMapping {
                data_key: "25".to_string(),
                chinese_description: "預警".to_string(),
                unit: "".to_string(),
            },
            DataMapping {
                data_key: "26".to_string(),
                chinese_description: "運行狀態".to_string(),
                unit: "".to_string(),
            },
            DataMapping {
                data_key: "27".to_string(),
                chinese_description: "計時".to_string(),
                unit: "".to_string(),
            },
            DataMapping {
                data_key: "28".to_string(),
                chinese_description: "故障".to_string(),
                unit: "".to_string(),
            },
            DataMapping {
                data_key: "32".to_string(),
                chinese_description: "運轉電壓".to_string(),
                unit: "V".to_string(),
            },
            DataMapping {
                data_key: "33".to_string(),
                chinese_description: "運轉電流".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "34".to_string(),
                chinese_description: "馬達輸出頻率".to_string(),
                unit: "Hz".to_string(),
            },
            DataMapping {
                data_key: "35".to_string(),
                chinese_description: "當前運轉功率".to_string(),
                unit: "kW".to_string(),
            },
            DataMapping {
                data_key: "36".to_string(),
                chinese_description: "風機輸出電壓".to_string(),
                unit: "V".to_string(),
            },
            DataMapping {
                data_key: "37".to_string(),
                chinese_description: "風機輸出電流".to_string(),
                unit: "A".to_string(),
            },
            DataMapping {
                data_key: "38".to_string(),
                chinese_description: "風機輸出頻率".to_string(),
                unit: "Hz".to_string(),
            },
            DataMapping {
                data_key: "39".to_string(),
                chinese_description: "風機輸出功率".to_string(),
                unit: "kW".to_string(),
            },
            DataMapping {
                data_key: "40".to_string(),
                chinese_description: "主機轉速".to_string(),
                unit: "RPM".to_string(),
            },
            DataMapping {
                data_key: "41".to_string(),
                chinese_description: "散熱風扇轉速".to_string(),
                unit: "RPM".to_string(),
            },
            DataMapping {
                data_key: "44".to_string(),
                chinese_description: "本次運行時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "47".to_string(),
                chinese_description: "本次負載時間".to_string(),
                unit: "hr".to_string(),
            },
            DataMapping {
                data_key: "61".to_string(),
                chinese_description: "主機本次用電".to_string(),
                unit: "kW·h".to_string(),
            },
            DataMapping {
                data_key: "66".to_string(),
                chinese_description: "風機本次用電".to_string(),
                unit: "kW·h".to_string(),
            },
            DataMapping {
                data_key: "77".to_string(),
                chinese_description: "每分鐘流量".to_string(),
                unit: "NL".to_string(),
            },
            DataMapping {
                data_key: "80".to_string(),
                chinese_description: "累積流量".to_string(),
                unit: "NM³".to_string(),
            },
        ];

        MockDatabase {
            gateways,
            brands,
            device_types,
            models,
            data,
            data_mapping,
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
