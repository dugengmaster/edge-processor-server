mod mqtt_client;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::actor::Actor;
use rumqttc::{Event, Incoming};
use rumqttc::{AsyncClient, MqttOptions as MOptions, QoS};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use v0::actor::data_actor::{DataMessage, SensorData};
use v0::actor::{data_actor::DataActor, publish_actor::PublishActor, RouterActor, RouterMessage};
use v0::gateway_modbus_device::MockDatabase;
use v0::message_processor::message::{DataPayload, Message};
use v0::message_processor::parser::ChannelType;
use v0::message_processor::MessageProcessor;
use std::time::Duration;
use std::error::Error;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions1 = MOptions::new("IoT_Core", "60.250.246.123", 1883);
    mqttoptions1.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let(mqtt_client1, mut eventloop1) = AsyncClient::new(mqttoptions1, 10);
    mqtt_client1.subscribe("DM/#", QoS::AtLeastOnce).await;

    // let mqttoptions2 = MqttOptions::new("processor2", "60.250.246.123", 1884);

    // let mut mqtt_client2 = RumqttClient::new(mqttoptions2);

    let mut mqttoptions2 = MOptions::new("mqtt_processor2", "60.250.246.123", 1884);
    mqttoptions2.set_keep_alive(Duration::from_secs(5));

    let (mqtt_client2, _) = AsyncClient::new(mqttoptions2, 10);

    let message_processor = Arc::new(MessageProcessor::new());
    // let (router_actor, _) = Actor::spawn(Some("router_actor".to_string()), RouterActor, ())
    //     .await
    //     .expect("Route Actor failed to start");
    // Actor::spawn(Some("data_actor".to_string()), DataActor, ())
    //     .await
    //     .expect("Data Actor failed to start");
    // Actor::spawn(Some("publish_actor".to_string()), PublishActor, ())
    //     .await
    //     .expect("Publish Actor failed to start");

    // let (tx, mut rx) = mpsc::channel(100);

    // tokio::spawn(async move {
    //     while let Some((topic, payload)) = rx.recv().await {
    //         match mqtt_client2.publish(topic, QoS::AtLeastOnce, false, payload).await {
    //             Ok(_) => {
    //                 println!("publish topic: {:?} sucessful.", &topic);
    //             }
    //             Err(e) => {
    //                 eprintln!("Failed to publish to topic {}: {:?}", topic, e);
    //             }
    //         };
    //     }
    // });

    loop {
        let event = eventloop1.poll().await;
        match &event {
            Ok(notif) => {
                println!("Event = {notif:?}");
            }
            Err(error) => {
                println!("Error = {error:?}");
                return Ok(());
            }
        }

        if let Ok(Event::Incoming(Incoming::Publish(publish))) = event {
            // this time we will ack incoming publishes.
            // Its important not to block eventloop as this can cause deadlock.
            let c = mqtt_client1.clone();
            let processor = message_processor.clone();
            tokio::spawn(async move {
                c.ack(&publish).await.unwrap();
                // match processor.message_processor(raw_message)
            });
        }
    }
    
        // .(move |raw_message| {
        //     let processor = message_processor.clone();
        //     let tx_clone = tx.clone();
        //     // let router = router_actor.clone();
        //     tokio::spawn(async move {
        //         match processor.message_processor(raw_message) {
        //             Ok(message) => {
        //                 match ChannelType::check_channel(message.topic.channel.as_str()) {
        //                     Ok(ChannelType::Data) => {
        //                         // 當訊息類型是 Data 時的處理邏輯
        //                         // 因為 PayloadType 是動態分發的 trait object,但我們需要具體的 DataPayload 型別
        //                         // 這裡使用 unsafe 來進行型別轉換,因為:
        //                         // 1. 我們確定在 Data channel 中的 payload 一定是 DataPayload
        //                         // 2. Box 的記憶體布局是相容的
        //                         // 3. 這個轉換不會造成記憶體洩漏或重複釋放
        //                         let data_message = Message {
        //                             topic: message.topic,
        //                             payload: unsafe {
        //                                 Box::from_raw(
        //                                     Box::into_raw(message.payload) as *mut DataPayload
        //                                 )
        //                             },
        //                         };
        //                         println!(
        //                             "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
        //                             data_message.topic.device_type,
        //                             data_message.topic.mac_id,
        //                             data_message.topic.channel,
        //                         );

        //                         // 創建模擬資料庫實例
        //                         let db = MockDatabase::new();

        //                         // 創建一個新的結構化數據集合
        //                         let mut sensor_data_vec: Vec<SensorData> = Vec::new();

        //                         // 遍歷原始資料
        //                         for (key, value) in &data_message.payload.data {
        //                             // 尋找對應的中文描述和單位
        //                             if let Some(mapping) = db
        //                                 .data_mapping
        //                                 .iter()
        //                                 .find(|mapping| mapping.data_key == *key)
        //                             {
        //                                 // 從 Value 中提取數值
        //                                 if let Some(num_value) = extract_number_value(value) {
        //                                     // 創建結構化的傳感器資料，使用預設的單位
        //                                     let sensor_data = SensorData {
        //                                         sensor_name_chinese: mapping
        //                                             .chinese_description
        //                                             .clone(),
        //                                         sensor_name_english: mapping
        //                                             .eng_description
        //                                             .clone(),
        //                                         value: num_value,
        //                                         timestamp: data_message.payload.timestamp.clone(),
        //                                         unit: mapping.unit.clone(),
        //                                     };

        //                                     // 添加到集合中
        //                                     sensor_data_vec.push(sensor_data);
        //                                 }
        //                             }

        //                             match serde_json::to_string(&sensor_data_vec) {
        //                                 Ok(json) => {
        //                                     // println!("payload: {}", json);
        //                                     tx_clone.send(("DM/skh", json)).await.unwrap();
        //                                 }
        //                                 Err(e) => {
        //                                     println!("Error serializing to JSON: {}", e);
        //                                 }
        //                             }
        //                         }
        //                     }
        //                     Ok(ChannelType::Ota) => {
        //                         println!("[INFO] OTA message received");
        //                     }
        //                     Err(e) => {
        //                         println!(
        //                             "[ERROR] Unknown channel type: {}, {:?}",
        //                             message.topic.channel, e
        //                         );
        //                     }
        //                 }
        //             }
        //             Err(err) => {
        //                 println!("Error processing message: {:?}", err);
        //                 return;
        //             } // Ok(message) => match router.cast(RouterMessage::Message(message)) {
        //               //     Ok(_) => {}
        //               //     Err(e) => println!("Failed to send message to router: {:?}", e),
        //               // },
        //               // Err(err) => {
        //               //     println!("Error processing message: {:?}", err);
        //               //     return;
        //               // }
        //         }
        //     });
        // })
        // .await;
}

fn extract_number_value(value: &Value) -> Option<f64> {
    if let Value::Number(num) = value {
        num.as_f64()
    } else if let Value::String(s) = value {
        s.parse::<f64>().ok()
    } else {
        None
    }
}
