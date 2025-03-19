use super::{MqttClient, MqttOptions};
use crate::v0::message_processor::message::RawMessage;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions as RumqttMqttOptions, Packet, QoS};
use std::time::Duration;

pub struct RumqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
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

        let (client, eventloop) = AsyncClient::new(rumqtt_options, 10);
        RumqttClient { client, eventloop }
    }

    async fn subscribe(&mut self, topic: &str) {
        self.client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    }

    // 
    async fn poll(&mut self, callback: impl Fn(RawMessage) + Send + Sync + 'static) {
        loop {
            match self.eventloop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(publish)) = notification {
                        let raw_message = RawMessage::from(publish);
                        callback(raw_message);
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    async fn publish(&mut self, topic: &str, payload: &[u8]) {
        if let Err(e) = self.client.publish(topic, QoS::AtLeastOnce, false, payload).await {
            eprintln!("Failed to publish message: {}", e);
        }
    }
}
