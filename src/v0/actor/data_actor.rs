use ractor::{Actor, ActorProcessingErr, ActorRef};
use super::ActorMessage;

pub struct DataActor;

impl Actor for DataActor {
    type Msg = ActorMessage;
    type State = u8;
    type Arguments = ();

    async fn pre_start(
        &self,
        _myself: ActorRef<Self::Msg>,
        _arguments: Self::Arguments,
    ) -> Result<Self::State, ActorProcessingErr> {
        Ok(0u8)
    }

    fn handle(
        &self,
        _myself: ActorRef<Self::Msg>,
        message: Self::Msg,
        _state: &mut Self::State,
    ) -> impl std::future::Future<Output = Result<(), ActorProcessingErr>> + Send {
        async move {
            match message {
                ActorMessage::Data(data) => {
                    println!("[INFO] Data received: {:?}", data);
                }
                ActorMessage::OTA(ota) => {
                    println!("[INFO] OTA received: {:?}", ota);
                }
            }
            Ok(())
        }
    }
}