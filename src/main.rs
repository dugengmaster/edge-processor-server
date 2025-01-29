mod mqtt_client;
use std::sync::Arc;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use v0::message_handler::MessageHandler;
use v0::message_processor::MessageProcessor;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("IoT_Core", "60.250.246.123", 1883);
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

    let mut mqtt_client = RumqttClient::new(mqttoptions);
    let message_handler = MessageHandler::new();
    let message_processor = Arc::new(MessageProcessor::new());

    mqtt_client.subscribe("DM/#").await;

    mqtt_client
        .poll(move |raw_message| {
            let handler = message_handler.clone();
            let task_message_processor = message_processor.clone();
            tokio::spawn(async move {
                handler.handle_message(raw_message, task_message_processor.clone()).await;
            });
        })
        .await;
}
