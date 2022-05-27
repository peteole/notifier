use std::{collections::HashMap, time};

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
#[derive(Debug,Clone)]
pub struct TelegramService {
    bot: AutoSend<Bot>,
    request_sender: Sender<(String, mpsc::Sender<ChatId>)>,
    stop_request_sender: Sender<String>,
    stop_sender: Sender<()>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TelegramConfig {
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
    pub async fn get_chat_id(&self, username: String) -> Result<ChatId, String> {
        //let (tx, rx) = mpsc::channel(10);
        let (sender, mut receiver) = mpsc::channel(10);
        self.request_sender
            .send((username.clone(), sender))
            .unwrap();
        match tokio::time::timeout(time::Duration::from_secs(60), receiver.recv()).await {
            Ok(a) => {
                return Ok(a.unwrap());
            }
            Err(_) => {
                self.stop_request_sender.send(username).unwrap();
                return Err("timeout".to_string());
            }
        }
    }
    pub fn load(serialized: String) -> Self {
        let config: TelegramConfig = serde_json::from_str(&serialized).unwrap();
        let raw_bot = Bot::new(config.token);
        let bot = raw_bot.clone().auto_send();
        let (request_sender, mut request_receiver) = channel(10);
        let (stop_request_sender, mut stop_request_receiver) = channel(10);
        let (stop_sender, mut stop_receiver) = channel(10);
        //let update_bot = bot.clone();
        //start a thread to handle username lookups
        tokio::spawn(async move {
            let mut offset = 0;
            let mut username_lookups: HashMap<String, mpsc::Sender<ChatId>> = HashMap::new();
            let mut interval = tokio::time::interval(time::Duration::from_secs(3));
            loop {
                select! {
                    req=request_receiver.recv() => {
                        match req{
                            Ok((username, sender))=>{
                                username_lookups.insert(username, sender);
                            }
                            Err(e)=>print!("Error receiving username lookup request: {}", e)
                        }
                      },
                      _=interval.tick()=>{
                        if username_lookups.len()==0{
                            continue;
                        }
                        //check for messages here
                        let updates=requests::JsonRequest::new(raw_bot.clone(),payloads::GetUpdates::new().offset(offset)).send().await.unwrap();
                        //let updates = update_bot.get_updates().await.unwrap();
                        for update in updates.iter() {
                            offset+=1;
                            let chat = update.chat().unwrap();
                            let username = chat.username().unwrap();
                            match username_lookups
                                .get(&username.to_string())
                            {
                                Some(sender) => {
                                    println!("Found user: {}", chat.id);
                                    sender.send(chat.id).await.unwrap();
                                    username_lookups.remove(&username.to_string());
                                }
                                None => {}
                            }
                        }

                    },
                    username=stop_request_receiver.recv()=>{
                        let username = username.unwrap();
                        username_lookups.remove(&username);
                    },
                    _=stop_receiver.recv() => {
                        println!("Stopping username lookup thread");
                        return ;
                    }
                }
            }
        });
        TelegramService {
            bot: bot,
            request_sender: request_sender,
            stop_request_sender: stop_request_sender,
            stop_sender: stop_sender,
        }
    }
    pub const ID: &'static str="telegram";
}
impl Drop for TelegramService {
    fn drop(&mut self) {
        self.stop_sender.send(()).unwrap();
    }
}
