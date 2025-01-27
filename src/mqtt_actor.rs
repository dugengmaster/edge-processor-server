use actix::prelude::*;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

use crate::v0::message_handler::MessageHandler;
use crate::v0::message_processor::message::RawMessage;

// Actor 消息
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe(pub String);

// MQTT Actor
pub struct MqttActor {
    client: Option<AsyncClient>,
    message_handler: MessageHandler,
}

impl Actor for MqttActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("MqttActor is started");
    }
}

impl MqttActor {
    pub fn new() -> Self {
        Self {
            client: None,
            message_handler: MessageHandler::new(),
        }
    }

    async fn setup_mqtt(&mut self) -> Result<AsyncClient, Box<dyn std::error::Error>> {
        let mut mqttoptions = MqttOptions::new("edge_server", "60.250.246.123", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(10));
        mqttoptions.set_credentials("dolomannaiot", "q03KHNrJG0wC");

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
        self.client = Some(client.clone());

        // 啟動事件循環
        let handler = self.message_handler.clone();
        tokio::spawn(async move {
            Self::handle_mqtt_events(eventloop, handler).await;
        });

        Ok(client)
    }

    async fn handle_mqtt_events(
        mut eventloop: rumqttc::EventLoop,
        handler: MessageHandler,
    ) {
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    if let Event::Incoming(Packet::Publish(publish)) = notification {
                        let handler = handler.clone();
                        tokio::spawn(async move {
                            let raw_message = RawMessage::from(publish);
                            handler.handle_message(raw_message).await;
                        });
                    }
                }
                Err(e) => {
                    println!("MQTT Event Error: {:?}", e);
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

// 處理連接消息
impl Handler<Connect> for MqttActor {
    type Result = ();

    fn handle(&mut self, _msg: Connect, ctx: &mut Context<Self>) {
        // 使用 actix 的 fut::wrap_future 來處理異步操作
        let fut = self.setup_mqtt();
        let fut = async move {
            match fut.await {
                Ok(_) => println!("Connected to MQTT broker"),
                Err(e) => println!("Failed to connect: {:?}", e),
            }
        };
        ctx.spawn(fut::wrap_future(fut));
    }
}

// 處理訂閱消息
impl Handler<Subscribe> for MqttActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, ctx: &mut Context<Self>) {
        if let Some(client) = &self.client {
            let client = client.clone();
            let topic = msg.0;
            let fut = async move {
                match client.subscribe(&topic, QoS::AtMostOnce).await {
                    Ok(_) => println!("Subscribed to {}", topic),
                    Err(e) => println!("Subscribe error: {:?}", e),
                }
            };
            ctx.spawn(fut::wrap_future(fut));
        }
    }
}