use crate::v0::message_processor::message::Message as MqttMessage;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use super::message_processor::message::PayloadType;

pub enum RouterMessage {
    Message(MqttMessage<dyn PayloadType>),
}

pub struct RouterActor;

impl Actor for RouterActor {
    type Msg = RouterMessage;
    type State = ();
    type Arguments = ();

    async fn pre_start(
        &self,
        myself: ActorRef<Self::Msg>,
        args: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(())
    }

    async fn handle(
        &self, 
        myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            RouterMessage::Message(message) => match message.topic.channel.as_str() {
                "1" => {
                    println!(
                        "[INFO] Message received - Type: {}, MAC: {}, Channel: {}, payload: {:?}",
                        message.topic.device_type,
                        message.topic.mac_id,
                        message.topic.channel,
                        message.payload
                    );
                }
                "0" => {
                    println!("[INFO] OTA message received");
                }
                _ => {
                    println!("[ERROR] Unknown channel type: {}", message.topic.channel);
                }
            },
        }
        Ok(())
    }
}
