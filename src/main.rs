mod mqtt_client;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use std::sync::Arc;
use std::time::Instant;
use ractor::actor::Actor;
use v0::message_processor::MessageProcessor;
// use v0::actor::DataActor;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("IoT_Core", "60.250.246.123", 1883);
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let mut mqtt_client = RumqttClient::new(mqttoptions);
    let message_processor = Arc::new(MessageProcessor::new());
    // let (actor, actor_handle) = Actor::spawn(None, DataActor, ()).await.expect("Actor failed to start");

    mqtt_client.subscribe("DM/#").await;

    mqtt_client
        .poll(move |raw_message| {
            let processor = message_processor.clone();
            tokio::spawn(async move {
                // let start = Instant::now();
                let message = processor.message_processor(raw_message);

                println!(
                    "[INFO] Message received - Type: {}, MAC: {}, Channel: {}, payload: {:?}",
                    message.topic.device_type, message.topic.mac_id, message.topic.channel, message.payload
                );

                // let duration = start.elapsed();
                // println!("[INFO] Message processing time: {:?}\n", duration);
            });
        })
        .await;
}
