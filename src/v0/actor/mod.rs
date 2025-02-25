pub mod data_actor;

use ractor::registry;
use ractor::{Actor, ActorProcessingErr, ActorRef};

use super::message_processor::message::DataPayload;
use super::message_processor::{
    message::Message as MqttMessage, message::PayloadType, parser::ChannelType,
};
use data_actor::DataMessage;

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
            RouterMessage::Message(message) => {
                match ChannelType::check_channel(message.topic.channel.as_str()) {
                    Ok(ChannelType::Data) => {
                        // 當訊息類型是 Data 時的處理邏輯
                        // 因為 PayloadType 是動態分發的 trait object,但我們需要具體的 DataPayload 型別
                        // 這裡使用 unsafe 來進行型別轉換,因為:
                        // 1. 我們確定在 Data channel 中的 payload 一定是 DataPayload
                        // 2. Box 的記憶體布局是相容的
                        // 3. 這個轉換不會造成記憶體洩漏或重複釋放
                        let data_message = DataMessage::Message(MqttMessage {
                            topic: message.topic,
                            payload: unsafe {
                                Box::from_raw(Box::into_raw(message.payload) as *mut DataPayload)
                            },
                        });

                        // 嘗試從 Actor 註冊表中找到 data_actor 並發送訊息
                        if let Some(data_actor) = registry::where_is("data_actor".to_string()) {
                            let actor_ref: ActorRef<DataMessage> = data_actor.into();
                            if let Err(e) = actor_ref.cast(data_message) {
                                println!("[ERROR] Failed to send message to data_actor: {:?}", e);
                            }
                        } else {
                            println!("[ERROR] data_actor not found in registry!");
                        }
                    }
                    Ok(ChannelType::Ota) => {
                        println!("[INFO] OTA message received");
                    }
                    Err(e) => {
                        println!(
                            "[ERROR] Unknown channel type: {}, {:?}",
                            message.topic.channel, e
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
