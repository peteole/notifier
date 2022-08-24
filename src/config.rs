use config::File;
use serde::{Deserialize, Serialize};
use sqlx::postgres;
use std::io::Error;

use crate::services::{
    email::{self, EmailService},
    service::Service,
    telegram::{self, TelegramService},
};
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicesConfig {
    email: Option<email::EmailConfig>,
    telegram: Option<telegram::TelegramConfig>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    //services: HashMap<String, String>,
    connection_string: String,
    services: ServicesConfig,
}
#[derive(Clone)]
pub struct ServerConfig {
    pub pool: postgres::PgPool,
    pub telegram: Option<TelegramService>,
    pub email: Option<EmailService>,
}

impl ConfigFile {
    pub async fn to_config(self) -> ServerConfig {
        let pool = postgres::PgPoolOptions::new()
            .connect(self.connection_string.as_str())
            .await
            .unwrap();
        sqlx::migrate!().run(&pool).await.unwrap();
        ServerConfig {
            pool,
            telegram: self.services.telegram.map(|c| TelegramService::load(c)),
            email: self.services.email.map(|c| EmailService::load(c)),
        }
    }
    pub fn load(path: impl AsRef<std::path::Path>) -> Result<ConfigFile, Error> {
        match std::fs::read_to_string(path) {
            Ok(data) => Ok(serde_yaml::from_str(&data).unwrap()),
            Err(e) => Err(e),
        }
    }
    /// Load from all files named "config.*" in the current working directory and the root directory as well as environment variables prefixed with "NOTIFIER"
    pub fn load_from_std_locations() -> Self {
        config::Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(File::with_name("/config").required(false))
            .add_source(
                config::Environment::with_prefix("NOTIFIER")
                    .try_parsing(true)
                    .separator(".")
                    .prefix_separator("."),
            )
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}
impl ServerConfig {
    pub fn get_service(&self, service_id: String) -> Option<&(dyn Service + Sync)> {
        match service_id.as_str() {
            telegram::TelegramService::ID => {
                if let Some(service) = &self.telegram {
                    return Some(service);
                }
            }
            EmailService::ID => {
                if let Some(service) = &self.email {
                    return Some(service);
                }
            }
            _ => return None,
        }
        return None;
    }
}
