// mod data_actor;
// mod ota_actor;

// use crate::v0::message_processor::message::{DataPayload, Message as MqttMessage, OTAPayload};
// use ractor::{Actor, ActorProcessingErr, ActorRef, Message};

// pub enum ActorMessage {
//     Data(MqttMessage<DataPayload>),
//     OTA(MqttMessage<OTAPayload>),
// }

// pub struct RouterActor;

// impl Actor for RouterActor {
//     type Msg = ActorMessage;
//     type State = ();
//     type Arguments = ();

    // async fn pre_start(
    //     &self,
    //     _ctx: &mut Context<Self::Msg>,
    //     _args: Self::Arguments,
    // ) -> Result<Self::State, ractor::Error> {
    //     Ok(())
    // }

    // async fn handle(
    //     &self,
    //     _ctx: &mut Context<Self::Msg>,
    //     message: Self::Msg,
    //     _state: &mut Self::State,
    // ) -> Result<(), ractor::Error> {
    //      match message.topic.channel.as_str() {
    //                 "data" => {
    //                      let data_actor = DataActor;
    //                     let actor_ref = DataActor::spawn(None, data_actor, ()).await.unwrap().0;
    //                      actor_ref.cast(ActorMessage::Data(*message.payload.downcast::<crate::v0::message_processor::message::DataPayload>().unwrap())).unwrap();
    //                 },
    //                "ota"=>{
    //                     let ota_actor = OtaActor;
    //                      let actor_ref = OtaActor::spawn(None, ota_actor, ()).await.unwrap().0;
    //                     actor_ref.cast(ActorMessage::Ota(*message.payload.downcast::<crate::v0::message_processor::message::OTAPayload>().unwrap())).unwrap();
    //                },
    //                 _ => {
    //                    println!("[ERROR] Unknown channel type: {}", message.topic.channel);
    //                 }
    //             }
    //     Ok(())
    // }
// }
