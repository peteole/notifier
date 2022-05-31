use serde::{Deserialize, Serialize};
use sqlx::postgres;
use std::{collections::HashMap, hash::Hash, io::Error};

use crate::services::{
    email::EmailService,
    service::Service,
    telegram::{self, TelegramService},
};
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigFile {
    services: HashMap<String, String>,
    connection_string: String,
}
#[derive(Clone)]
pub struct Config {
    pub pool: postgres::PgPool,
    pub telegram: Option<TelegramService>,
    pub email: Option<EmailService>,
    // pub services: HashMap<String, Box<dyn Service>>,
}

impl ConfigFile {
    pub async fn to_config(self) -> Config {
        let pool = postgres::PgPoolOptions::new()
            .connect(self.connection_string.as_str())
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        let mut config = Config {
            pool,
            telegram: None,
            email: None,
            //services: HashMap::new(),
        };
        for (service_name, config_data) in self.services {
            match service_name.as_str() {
                telegram::TelegramService::ID => {
                    let service = TelegramService::load(config_data);
                    config.telegram = Some(service);
                    // config
                    //     .services
                    //     .insert(telegram::TelegramService::ID.to_string(), Box::new(service));
                }
                EmailService::ID => {
                    let service = EmailService::load(config_data);
                    config.email = Some(service);
                    // config
                    //     .services
                    //     .insert(EmailService::ID.to_string(), Box::new(service));
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
impl Config {
    pub fn get_service(&self, service_id: String) -> Option<&(dyn Service+Sync)> {
        match service_id.as_str() {
            telegram::TelegramService::ID => {
                if let Some(service) = &self.telegram {
                    return Some(service)
                } 
            },
            EmailService::ID => {
                if let Some(service) = &self.email {
                    return Some(service)
                } 
            },
            _ => return None,
        }
        return None;
    }
}

// impl Clone for Config {
//     fn clone(&self) -> Self {
//         let mut servicesClone:HashMap<String, Box<dyn Service>>= HashMap::new();
//         if let Some(telegram) = &self.telegram {
//             servicesClone.insert(telegram::TelegramService::ID.to_string(), Box::new(telegram.clone()));
//         }
//         if let Some(email) = &self.email {
//             servicesClone.insert(EmailService::ID.to_string(), Box::new(email.clone()));
//         }
//         Config {
//             pool: self.pool.clone(),
//             telegram: self.telegram.clone(),
//             email: self.email.clone(),
//             services: servicesClone,
//         }
//     }
// }
