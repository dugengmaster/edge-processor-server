use ractor::registry;
use ractor::{Actor, ActorProcessingErr, ActorRef};
use crate::mqtt_client::{rumqtt_client::RumqttClient, MqttClient, MqttOptions};

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
        let mut mqttoptions = MqttOptions::new("IoT_Processor", "60.250.246.123", 1884);
        let mut mqtt_client = RumqttClient::new(mqttoptions);

        Ok(mqtt_client)
    }

    async fn handle(
        &self,
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        state.publish("DM/processor", message).await;

        Ok(())
    }
}