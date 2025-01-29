use crate::v0::message_processor::message::RawMessage;
pub mod rumqtt_client;

pub trait MqttClient {
    fn new(mqttoptions: MqttOptions) -> Self;
    async fn subscribe(&mut self, topic: &str);
    async fn poll(&mut self, callback: impl Fn(RawMessage) + Send + Sync + 'static);
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct MqttOptions {
    pub server_address: String,
    pub port: u16,
    pub client_id: String,
    pub keep_alive: u64,
    pub username: Option<String>,
    pub password: String,
    pub clean_session: bool,
    pub will: Option<Will>,
    pub max_packet_size: u32,
    pub connect_timeout: u64,
    pub automatic_reconnect: bool,
    pub tls: Option<TlsOptions>,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct Will {
    pub topic: String,
    pub message: String,
    pub qos: u8,
    pub retain: bool,
}
#[allow(dead_code)]
#[derive(Clone)]
pub struct TlsOptions {
    pub ca_path: String,
    pub client_cert_path: Option<String>,
    pub client_key_path: Option<String>,
    pub insecure_skip_verify: bool,
}
#[allow(dead_code)]
impl MqttOptions {
    pub fn new(client_id: &str, server_address: &str, port: u16) -> Self {
        MqttOptions {
            server_address: server_address.to_string(),
            port,
            client_id: client_id.to_string(),
            keep_alive: 10,
            username: None,
            password: "".to_string(),
            clean_session: true,
            will: None,
            max_packet_size: 65535,
            connect_timeout: 30,
            automatic_reconnect: true,
            tls: None,
        }
    }

    pub fn set_credentials(&mut self, username: &str, password: &str) -> &mut Self {
        self.username = Some(username.to_string());
        self.password = password.to_string();
        self
    }

    pub fn set_client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = client_id.to_string();
        self
    }

    pub fn set_clean_session(&mut self, clean_session: bool) -> &mut Self {
        self.clean_session = clean_session;
        self
    }

    pub fn set_will(&mut self, topic: &str, message: &str, qos: u8, retain: bool) -> &mut Self {
        self.will = Some(Will {
            topic: topic.to_string(),
            message: message.to_string(),
            qos,
            retain,
        });
        self
    }

    pub fn set_max_packet_size(&mut self, max_packet_size: u32) -> &mut Self {
        self.max_packet_size = max_packet_size;
        self
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: u64) -> &mut Self {
        self.connect_timeout = connect_timeout;
        self
    }

    pub fn set_automatic_reconnect(&mut self, automatic_reconnect: bool) -> &mut Self {
        self.automatic_reconnect = automatic_reconnect;
        self
    }

    pub fn set_tls(
        &mut self,
        ca_path: &str,
        client_cert_path: Option<&str>,
        client_key_path: Option<&str>,
        insecure_skip_verify: bool,
    ) -> &mut Self {
        self.tls = Some(TlsOptions {
            ca_path: ca_path.to_string(),
            client_cert_path: client_cert_path.map(|s| s.to_string()),
            client_key_path: client_key_path.map(|s| s.to_string()),
            insecure_skip_verify,
        });
        self
    }
}
