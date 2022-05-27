use std::{collections::HashMap, io::Error};

use serde::{Deserialize, Serialize};

use crate::services::{
    email::EmailService,
    service::Service,
    telegram::{self, TelegramService},
};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    services: HashMap<String, String>,
}
pub struct Config {
    pub telegram: Option<TelegramService>,
    pub email: Option<EmailService>,
    services: HashMap<String, Box<dyn Service>>,
}

impl ConfigFile {
    pub fn toConfig(self) -> Config {
        let mut config = Config {
            telegram: None,
            email: None,
            services: HashMap::new(),
        };
        for (serviceName, configData) in self.services {
            match serviceName.as_str() {
                telegram::TelegramService::ID => {
                    let service = TelegramService::load(configData);
                    config.telegram = Some(service.clone());
                    config
                        .services
                        .insert(telegram::TelegramService::ID.to_string(), Box::new(service));
                }
                EmailService::ID => {
                    let service = EmailService::load(configData);
                    config.email = Some(service.clone());
                    config
                        .services
                        .insert(EmailService::ID.to_string(), Box::new(service));
                }
                _ => {}
            }
        }
        config
    }
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<ConfigFile, Error> {
        match std::fs::read_to_string(path) {
            Ok(data) => Ok(serde_yaml::from_str(&data).unwrap()),
            Err(e) => Err(e),
        }
    }
}
