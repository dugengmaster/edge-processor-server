pub mod channel;
pub mod device_message_handler;

use crate::device_message_handler::DeviceMessageHandler;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;
use redis::Client;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("test1", "60.250.246.123", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("DM/#", QoS::AtMostOnce).await.unwrap();

    let (message_handler, (mut data_receiver, mut status_receiver)) = DeviceMessageHandler::new();

    // 啟動數據消息處理器
    tokio::spawn(async move {
        while let Some(message) = data_receiver.recv().await {
            DeviceMessageHandler::handle_data_message(message).await;
        }
    });

    // 啟動狀態消息處理器
    tokio::spawn(async move {
        while let Some(message) = status_receiver.recv().await {
            DeviceMessageHandler::handle_status_message(message).await;
        }
    });

    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                if let Event::Incoming(Packet::Publish(publish)) = notification {
                    let handler = message_handler.clone();
                    tokio::task::spawn(async move {
                        handler.dispatch_message(&publish.topic, &publish.payload).await;
                    });
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}