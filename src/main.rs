mod mqtt_client;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::actor::Actor;
use std::sync::Arc;
use v0::actor::{data_actor::DataActor, RouterActor, RouterMessage};
use v0::message_processor::MessageProcessor;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("IoT_Core", "60.250.246.123", 1883);
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let mut mqtt_client = RumqttClient::new(mqttoptions);
    let message_processor = Arc::new(MessageProcessor::new());
    let (router_actor, _) = Actor::spawn(Some("router_actor".to_string()), RouterActor, ())
        .await
        .expect("Route Actor failed to start");
    Actor::spawn(Some("data_actor".to_string()), DataActor, ())
        .await
        .expect("Data Actor failed to start");

    mqtt_client.subscribe("DM/#").await;

    mqtt_client
        .poll(move |raw_message| {
            let processor = message_processor.clone();
            let router = router_actor.clone();
            tokio::spawn(async move {
                match processor.message_processor(raw_message) {
                    Ok(message) => {
                        router.cast(RouterMessage::Message(message)).unwrap();
                    }
                    Err(err) => {
                        println!("Error processing message: {:?}", err);
                        return;
                    }
                }
            });
        })
        .await;
}
