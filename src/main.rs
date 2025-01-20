mod device_message_controller;
mod device_message_handler;
mod device_model;
mod gateway_modbus_device;
mod message_processor;

use device_message_handler::MessageHandler;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;
use message_processor::message::RawMessage;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("edge_server", "60.250.246.123", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("DM/#", QoS::AtMostOnce).await.unwrap();

    let handler = MessageHandler::new();

    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                if let Event::Incoming(Packet::Publish(publish)) = notification {
                    let handler = handler.clone();
                    tokio::task::spawn(async move {
                        let raw_message = RawMessage::from(publish);
                        handler.handle_message(raw_message).await;
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
