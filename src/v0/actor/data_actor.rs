use crate::v0::message_processor::message::{DataPayload, Message as MqttMessage};
use ractor::{Actor, ActorProcessingErr, ActorRef};

pub enum DataMessage {
    Message(MqttMessage<DataPayload>),
}
pub struct DataActor;

impl Actor for DataActor {
    type Msg = DataMessage;
    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(0u8)
    }

    async fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> Result<(), ActorProcessingErr> {
        match message {
            DataMessage::Message(message) => {
                println!(
                    "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
                    message.topic.device_type, message.topic.mac_id, message.topic.channel,
                );
                println!("payload: {:?}", message.payload)
            }
        }
        Ok(())
    }
}
