use crate::v0::actor::publish_actor::PublishMessage;
use crate::v0::gateway_modbus_device::MockDatabase;
use crate::v0::message_processor::message::{DataPayload, Message as MqttMessage};
use ractor::{registry, Actor, ActorProcessingErr, ActorRef};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub enum DataMessage {
    Message(MqttMessage<DataPayload>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SensorData {
    pub sensor_name_chinese: String, // 感測器名稱（中文描述）
    pub sensor_name_english: String, // 感測器名稱（英文描述）
    pub value: f64,                  // 數值
    pub timestamp: String,           // 時間戳記
    pub unit: String,                // 單位
}

pub struct DataActor;

impl Actor for DataActor {
    type Msg = DataMessage;
    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(0u8)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            DataMessage::Message(message) => {
                println!(
                    "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
                    message.topic.device_type, message.topic.mac_id, message.topic.channel,
                );

                // 創建模擬資料庫實例
                let db = MockDatabase::new();

                // 創建一個新的結構化數據集合
                let mut sensor_data_vec: Vec<SensorData> = Vec::new();

                // 遍歷原始資料
                for (key, value) in &message.payload.data {
                    // 尋找對應的中文描述和單位
                    if let Some(mapping) = db
                        .data_mapping
                        .iter()
                        .find(|mapping| mapping.data_key == *key)
                    {
                        // 從 Value 中提取數值
                        if let Some(num_value) = extract_number_value(value) {
                            // 創建結構化的傳感器資料，使用預設的單位
                            let sensor_data = SensorData {
                                sensor_name_chinese: mapping.chinese_description.clone(),
                                sensor_name_english: mapping.eng_description.clone(),
                                value: num_value,
                                timestamp: message.payload.timestamp.clone(),
                                unit: mapping.unit.clone(),
                            };

                            // 添加到集合中
                            sensor_data_vec.push(sensor_data);
                        }
                    }
                }

                // 打印轉換後的資料
                // println!("payload: {:?}", sensor_data_vec);

                // 將數據序列化為 JSON 格式
                match serde_json::to_string(&sensor_data_vec) {
                    Ok(json) => {
                        // println!("payload: {}", json);

                        // 透過 registry 獲取 PublishActor 的引用
                        if let Some(publish_actor) = registry::where_is("publish_actor".to_string())
                        {
                            // 將 JSON 數據發送給 PublishActor
                            let publish_ref: ActorRef<PublishMessage> = publish_actor.into();
                            if let Err(e) = publish_ref.cast(PublishMessage::Message(json)) {
                                println!("[ERROR] Failed to send message to PublishActor: {:?}", e);
                            }
                        } else {
                            println!("[ERROR] PublishActor not found in registry");
                        }
                    }
                    Err(e) => {
                        println!("Error serializing to JSON: {}", e);
                    }
                }
            }
        }

        Ok(())
    }
}

// 從 Value 中提取數值
fn extract_number_value(value: &Value) -> Option<f64> {
    if let Value::Number(num) = value {
        num.as_f64()
    } else if let Value::String(s) = value {
        s.parse::<f64>().ok()
    } else {
        None
    }
}
