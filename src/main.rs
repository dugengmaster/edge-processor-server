mod v0;

use v0::message_handler::MessageHandler;
use v0::mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("60.250.246.123", 1883);
    mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");
    
    let message_handler = MessageHandler::new();
    let mut mqtt_client = RumqttClient::new(mqttoptions);
    
    mqtt_client.subscribe("DM/#").await;

    mqtt_client.poll(move |raw_message| {
        let handler = message_handler.clone();
        tokio::spawn(async move {
            handler.handle_message(raw_message).await;
        });
    }).await;
}
