mod services;
mod server;
mod config;
use services::{service::*, telegram::TelegramService};
#[tokio::main]
async fn main() {
    let config=config::ConfigFile::load("config.yaml").unwrap();
    let config=config.toConfig();
    //println!("{:?}", config);
    let telegram_service = config.telegram.unwrap();
    let chat_id = telegram_service
        .get_chat_id("peteole".to_string())
        .await
        .unwrap();
    telegram_service
        .send(
            chat_id.to_string(),
            "test".to_string(),
            "Hello world form rust!".to_string(),
        )
        .await;
}
