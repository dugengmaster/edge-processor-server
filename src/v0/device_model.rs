use crate::v0::gateway_modbus_device::*;

pub struct DeviceModel {
    database: MockDatabase,
}
#[allow(dead_code)]
impl DeviceModel {
    pub fn new() -> Self {
        DeviceModel {
            database: MockDatabase::new(),
        }
    }

    pub fn get_gateway_by_macid(&self, mac_id: &str) -> Option<&Gateway> {
        self.database
            .gateways
            .iter()
            .find(|gateway| &gateway.gateway_macid == mac_id)
    }

    pub fn get_brand_by_name(&self, brand_name: &str) -> Option<&Brand> {
        self.database
            .brands
            .iter()
            .find(|brand| &brand.brand_name == brand_name)
    }

    pub fn get_device_type_by_name(&self, device_type_name: &str) -> Option<&DeviceType> {
        self.database
            .device_types
            .iter()
            .find(|device_type| &device_type.device_type_name == device_type_name)
    }

    pub fn get_model_by_name_and_ids(
        &self,
        model_name: &str,
        brand_id: u32,
        device_type_id: u32,
    ) -> Option<&Model> {
        self.database.models.iter().find(|model| {
            &model.model_name == model_name
                && model.brand_id == brand_id
                && model.device_type_id == device_type_id
        })
    }

    pub fn get_device_info_by_ids(
        &self,
        gateway_id: u32,
        slaveid: u8,
        model_id: u32,
    ) -> Option<&DeviceInfo> {
        self.database.data.iter().find(|device_info| {
            device_info.gateway_id == gateway_id
                && device_info.slaveid == slaveid
                && device_info.model_id == model_id
        })
    }
}
