use ractor::{Actor, ActorProcessingErr, ActorRef};
// use crate::mqtt_client::{MqttClient, MqttOptions, rumqtt_client::RumqttClient};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::env;
use std::time::Duration;
use tokio::task;

pub enum PublishMessage {
    Message(String),
}

pub struct PublishActor;

impl Actor for PublishActor {
    type Msg = PublishMessage;
    type State = AsyncClient;
    // type State = RumqttClient;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let mut mqttoptions = MqttOptions::new(
            &env::var("MQTT2_CLIENT_ID").expect("MQTT2_CLIENT_ID must be set"),
            &env::var("MQTT2_BROKER_HOST").expect("MQTT2_BROKER_HOST must be set"),
            env::var("MQTT2_BROKER_PORT")
                .expect("MQTT1_BROKER_PORT must be set")
                .parse::<u16>()
                .expect("MQTT2_BROKER_PORT must be set"),
        );
        mqttoptions.set_keep_alive(Duration::from_secs(60));

        let (mqtt_client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

        let _ = mqtt_client.subscribe("DM/#", QoS::AtLeastOnce).await;

        task::spawn(async move {
            loop {
                if let Err(e) = eventloop.poll().await {
                    eprintln!("MQTT Error: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        // let mqttoptions = MqttOptions::new("mqtt_processor2", "60.250.246.123", 1884);
        // let mut mqtt_client = RumqttClient::new(mqttoptions);

        // mqtt_client.subscribe("DM/skh").await;

        // mqtt_client.poll().await;

        Ok(mqtt_client)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        // let publish_client = state.clone();
        match message {
            PublishMessage::Message(message) => {
                if let Err(e) = state
                    .publish("DM/skh", QoS::AtMostOnce, false, message)
                    .await
                {
                    println!("Error publishing: {:?}", e)
                }
            }
        }
        Ok(())
    }
}
