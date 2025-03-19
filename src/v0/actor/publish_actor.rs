use crate::mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};
use ractor::registry;
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub enum PublishMessage {
    Message(String),
}

pub struct PublishActor;

impl Actor for PublishActor {
    type Msg = PublishMessage;
    type State = RumqttClient;
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        let mqttoptions = MqttOptions::new("IoT_Processor", "60.250.246.123", 1884);
        let mut mqtt_client = RumqttClient::new(mqttoptions);
        mqtt_client.subscribe("DM/skh").await;

        Ok(mqtt_client)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        let PublishMessage::Message(message) = message;
        state.publish("DM/skh", message.as_bytes()).await;
        state
            .poll(move |raw_message| {
                tokio::spawn(async move {
                    if let Ok(message) = std::str::from_utf8(raw_message.payload) {
                        println!("[INFO] Message received: {}", message);
                    }
                });
            })
            .await;

        Ok(())
    }
}
