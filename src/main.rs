mod mqtt_client;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::actor::{self, Actor};
use std::sync::Arc;
use v0::message_processor::MessageProcessor;
use v0::actor::{RouterActor, RouterMessage};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("IoT_Core", "60.250.246.123", 1883);
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let mut mqtt_client = RumqttClient::new(mqttoptions);
    let message_processor = Arc::new(MessageProcessor::new());
    let (router_actor, _) = Actor::spawn(None, RouterActor, ()).await.expect("Actor failed to start");

    mqtt_client.subscribe("DM/#").await;

    mqtt_client
        .poll(move |raw_message| {
            let processor = message_processor.clone();
            let router = router_actor.clone();
            tokio::spawn(async move {
                let message = processor.message_processor(raw_message);
                router.cast(RouterMessage::Message(message)).unwrap();
            });
        })
        .await;
}
