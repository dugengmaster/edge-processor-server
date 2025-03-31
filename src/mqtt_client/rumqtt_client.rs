use super::{MqttClient, MqttOptions};
use crate::v0::message_processor::message::RawMessage;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions as RumqttMqttOptions, Packet, QoS};
use std::error::Error;
use std::time::Duration;

pub struct RumqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
    message_hook: Option<Box<dyn Fn(RawMessage) + Send + Sync + 'static>>,
}

impl MqttClient for RumqttClient {
    fn new(mqttoptions: MqttOptions) -> Self {
        let mut rumqtt_options = RumqttMqttOptions::new(
            mqttoptions.client_id,
            mqttoptions.server_address,
            mqttoptions.port,
        );

        rumqtt_options.set_keep_alive(Duration::from_secs(mqttoptions.keep_alive));
        if let Some(username) = &mqttoptions.username {
            rumqtt_options.set_credentials(username, mqttoptions.password);
        }

        let (client, eventloop) = AsyncClient::new(rumqtt_options, 5);
        RumqttClient {
            client,
            eventloop,
            message_hook: None,
        }
    }

    async fn subscribe(&mut self, topic: &str) {
        self.client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    }

    fn set_message_hook(&mut self, hook: impl Fn(RawMessage) + Send + Sync + 'static) {
        self.message_hook = Some(Box::new(hook));
    }

    //
    async fn poll(&mut self) {
        loop {
            match self.eventloop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(publish)) = notification {
                        let raw_message = RawMessage::from(publish);
                        // 如果有註冊 hook，則調用它
                        if let Some(hook) = &self.message_hook {
                            hook(raw_message);
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    async fn publish(&mut self, topic: &str, payload: String) -> Result<(), Box<dyn Error>> {
        match self
            .client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to publish message: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}
