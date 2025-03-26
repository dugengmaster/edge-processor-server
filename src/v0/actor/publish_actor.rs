// use crate::mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::{Actor, ActorProcessingErr, ActorRef};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

pub enum PublishMessage {
    Message(String),
}

pub struct PublishActor;

impl Actor for PublishActor {
    type Msg = PublishMessage;
    type State = AsyncClient;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let mut mqttoptions = MqttOptions::new("mqtt_processor2", "60.250.246.123", 1884);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        // mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

        let (mqtt_client, _) = AsyncClient::new(mqttoptions, 5);

        // let mqttoptions = MqttOptions::new("IoT_Processor", "60.250.246.123", 1884);
        // let mut mqtt_client = RumqttClient::new(mqttoptions);

        // let mut mqttoptions = MqttOptions::new("IoT_Co", "60.250.246.123", 1883);
        // mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

        // let mut mqtt_client = RumqttClient::new(mqttoptions);

        // mqtt_client.subscribe("DM/skh").await;

        // mqtt_client.poll(|raw_message| {
        //     if let Ok(s) = std::str::from_utf8(&raw_message.payload) {
        //         println!("{}", s);
        //     } else {
        //         println!("Invalid UTF-8 data");
        //     }
        // });

        Ok(mqtt_client)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            PublishMessage::Message(message) => {
                // println!("Publishing message: {}", message); // 打印要发布的消息
                // match state.publish("DM/skh", message.as_bytes()).await {
                //     Ok(_) => {
                //         println!("\npublish done");
                //         println!("Publishing message: {}", message);
                //     }
                //     Err(e) => println!("Error publishing: {:?}", e), // 打印发布错误信息
                // }
                // let payload = format!("{{\"message\": \"{}\"}}", "data");
                match state
                    .publish("test/skh", QoS::AtLeastOnce, false, message)
                    .await
                {
                    Ok(_) => {
                        println!("\npublish done");
                    }
                    Err(e) => {
                        // 詳細檢查錯誤類型
                        match e {
                            rumqttc::ClientError::Request(req) => {
                                println!("{:?}", req)
                            }
                            _ => {
                                eprintln!("Unknown error: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
        // match message {
        //     PublishMessage::Message(message) => {
        //         let chunk_size = 1024; // 每個 chunk 的大小 (bytes)
        //         let chunks: Vec<&[u8]> = message.as_bytes().chunks(chunk_size).collect();

        //         for (i, chunk) in chunks.iter().enumerate() {
        //             // 建立一個包含 chunk 和序列號的訊息
        //             let chunk_message = format!(
        //                 "{{\"seq\":{}, \"data\":\"{}\"}}",
        //                 i,
        //                 String::from_utf8_lossy(chunk)
        //             );

        //             match state
        //                 .publish("DM/skh", QoS::AtMostOnce, false, chunk_message.as_bytes())
        //                 .await
        //             {
        //                 Ok(_) => {
        //                     println!("\npublish chunk {} done", i);
        //                 }
        //                 Err(e) => {
        //                     println!("Error publishing chunk {}: {:?}", i, e);
        //                     // 你可以在這裡添加重試機制或錯誤處理
        //                 }
        //             }
        //         }
        //     }
        // }
        Ok(())
    }
}
