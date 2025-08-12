mod mqtt_client;
mod v0;

use mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::actor::Actor;
use std::env;
use std::sync::Arc;
use v0::actor::{data_actor::DataActor, publish_actor::PublishActor, RouterActor, RouterMessage};
use v0::message_processor::MessageProcessor;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mut mqttoptions = MqttOptions::new(
        &env::var("MQTT1_CLIENT_ID").unwrap_or_else(|_| "IoT_Core".to_string()),
        &env::var("MQTT1_BROKER_HOST").expect("MQTT1_BROKER_HOST must be set"),
        env::var("MQTT1_BROKER_PORT")
            .expect("MQTT1_BROKER_PORT must be set")
            .parse::<u16>().expect("MQTT1_BROKER_PORT must be a valid port number"),
    );
    mqttoptions.set_credentials(
        &env::var("MQTT1_USERNAME").expect("MQTT1_USERNAME must be set"),
        &env::var("MQTT1_PASSWORD").expect("MQTT1_PASSWORD must be set"),
    );

    let mut mqtt_client = RumqttClient::new(mqttoptions);
    let message_processor = Arc::new(MessageProcessor::new());
    let (router_actor, _) = Actor::spawn(Some("router_actor".to_string()), RouterActor, ())
        .await
        .expect("Route Actor failed to start");
    Actor::spawn(Some("data_actor".to_string()), DataActor, ())
        .await
        .expect("Data Actor failed to start");
    Actor::spawn(Some("publish_actor".to_string()), PublishActor, ())
        .await
        .expect("Publish Actor failed to start");

    mqtt_client
        .subscribe(&env::var("MQTT1_TOPIC").expect("MQTT1_TOPIC must be set"))
        .await;

    mqtt_client.set_message_hook(move |raw_message| {
        let processor = message_processor.clone();
        let router = router_actor.clone();
        tokio::spawn(async move {
            match processor.message_processor(raw_message) {
                Ok(message) => match router.cast(RouterMessage::Message(message)) {
                    Ok(_) => {}
                    Err(e) => println!("Failed to send message to router: {:?}", e),
                },
                Err(err) => {
                    println!("Error processing message: {:?}", err);
                    return;
                }
            }
        });
    });

    mqtt_client.poll().await;
}
