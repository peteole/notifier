use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time,
};

use super::service;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use teloxide::{payloads, prelude::*, requests, types::ChatId};
use tokio::{
    select,
    sync::{
        broadcast::{channel, Sender},
        mpsc,
    },
};
#[derive(Debug, Clone)]
pub struct TelegramService {
    bot: AutoSend<Bot>,
    username_lookup_service: Arc<UsernameLookupService>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramConfig {
    token: String,
}
#[async_trait]
impl service::Service for TelegramService {
    async fn send(&self, receiver: String, subject: String, message: String) {
        println!("Sending message to {}: {}", receiver, message);
        self.bot
            .send_message(
                ChatId(receiver.parse().unwrap()),
                format!("{}\n{}", subject, message),
            )
            .await
            .unwrap();
    }
}

impl TelegramService {
    pub async fn get_chat_id(&mut self, username: String) -> Option<ChatId> {
        self.username_lookup_service.lookup_username(username).await
    }
    pub fn load(config: TelegramConfig) -> Self {
        //let config: TelegramConfig = serde_json::from_str(&serialized).unwrap();
        let raw_bot = Bot::new(config.token);
        let bot = raw_bot.clone().auto_send();
        TelegramService {
            bot: bot,
            username_lookup_service: Arc::new(UsernameLookupService::new(raw_bot)),
        }
    }
    pub const ID: &'static str = "telegram";
}
#[derive(Debug, Clone)]
pub struct UsernameLookupService {
    //bot: AutoSend<Bot>,
    username_lookup_table: Arc<Mutex<HashMap<String, mpsc::Sender<ChatId>>>>,
    stop_sender: Sender<()>,
}

impl UsernameLookupService {
    pub fn new(bot: Bot) -> Self {
        let u: Arc<Mutex<HashMap<String, mpsc::Sender<ChatId>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let username_lookups = Arc::clone(&u);
        let (stop_sender, mut stop_receiver) = channel(10);
        tokio::spawn(async move {
            let mut offset = 0;
            let mut interval = tokio::time::interval(time::Duration::from_secs(3));
            loop {
                select! {
                    _=interval.tick()=>{
                        if username_lookups.lock().unwrap().len() == 0 {
                            continue;
                        }
                        //check for messages here
                        let updates = requests::JsonRequest::new(
                            bot.clone(),
                            payloads::GetUpdates::new().offset(offset),
                        )
                        .send()
                        .await
                        .unwrap();
                        for update in updates.iter() {
                            offset += 1;
                            let chat = update.chat().unwrap();
                            let username = chat.username().unwrap();
                            let o=username_lookups.lock().unwrap().get(&username.to_string()).map(|s| s.clone());
                            if let Some(sender) =
                                o
                            {
                                sender.send(chat.id).await.unwrap();
                                username_lookups
                                    .lock()
                                    .unwrap()
                                    .remove(&username.to_string());
                            }
                        }
                    }

                    _=stop_receiver.recv() => {
                        println!("Stopping username lookup thread");
                        return ;
                    }
                }
            }
        });
        UsernameLookupService {
            username_lookup_table: u,
            stop_sender: stop_sender,
        }
    }
    pub async fn lookup_username(&self, username: String) -> Option<ChatId> {
        let (sender, mut receiver) = mpsc::channel(10);
        self.username_lookup_table
            .lock()
            .unwrap()
            .insert(username, sender);
        receiver.recv().await
    }
}

impl Drop for UsernameLookupService {
    fn drop(&mut self) {
        if let Err(e) = self.stop_sender.send(()) {
            print!("{}", e)
        }
    }
}
