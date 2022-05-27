use async_trait::async_trait;
use lettre::{self, transport::smtp::authentication::Credentials, transport::Transport};
use serde::{Deserialize, Serialize};

use super::service::Service;
#[derive(Clone)]
pub struct EmailService {
    client: lettre::SmtpTransport,
    sender: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailConfig {
    pub smtp_relay: String,
    pub username: String,
    pub password: String,
    pub sender: String,
}

#[async_trait]
impl Service for EmailService {
    async fn send(&self, receiver: String, subject: String, message: String) {
        println!("Sending message to {}: {}", receiver, message);
        let email = lettre::Message::builder()
            .from(self.sender.parse().unwrap())
            .to(receiver.parse().unwrap())
            .subject(subject)
            .body(message)
            .unwrap();
        self.client.send(&email).unwrap();
    }
}
impl EmailService {
    pub fn load(serialized: String) -> Self {
        let config: EmailConfig = serde_json::from_str(&serialized).unwrap();
        let credentials = Credentials::new(config.username, config.password);
        let client = lettre::SmtpTransport::relay(config.smtp_relay.as_str())
            .unwrap()
            .credentials(credentials)
            .build();
        EmailService {
            client: client,
            sender: config.sender,
        }
    }
    pub const ID: &'static str="email";
}
