use crate::application::error::*;
use config::{Config, File};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub database_url: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: i64,
    pub smpt: SmtpConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub admin_email: String,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        let settings = Config::builder()
            .add_source(File::with_name("config.toml").required(true))
            .build()?;
        Ok(settings.try_deserialize()?)
    }
}
