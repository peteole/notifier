/// A service is an object that can send messages to a receiver.
use async_trait::async_trait;

#[async_trait]
pub trait Service {
    async fn send(&self, receiver: String, subject: String, message: String);
    // fn load(serialized: String) -> Self;
    // /// returns a unique name for this service
    // fn id() -> String;
}
