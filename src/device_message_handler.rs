use crate::device_message_controller::DeviceMessageController;
use rumqttc::Publish;

#[derive(Clone)]
pub struct MessageHandler;

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler
    }

    pub async fn handle_message(&self, publish: Publish) {
        let topic = publish.topic;
        let payload = publish.payload.to_vec();

        if let Some(message) = DeviceMessageController::new(topic, payload) {
            println!(
                "[INFO] Message received - Type: {}, MAC: {}, Channel: {}",
                message.device_type, message.mac_id, message.channel
            );
        }
    }
}