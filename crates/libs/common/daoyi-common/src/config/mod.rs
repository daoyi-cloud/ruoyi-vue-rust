mod auth;
mod database;
mod nacos;
mod redis;
mod server;
mod sms_code;
mod tenant;

use crate::config::nacos::load_nacos_config;
use crate::config::sms_code::SmsCodeConfig;
use anyhow::{Context, anyhow};
pub use auth::AuthConfig;
use config::{Config, FileFormat};
pub use database::DatabaseConfig;
pub use redis::RedisConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::{Arc, LazyLock};
pub use tenant::TenantConfig;
use tokio::sync::RwLock;

// static CONFIG: OnceCell<AppConfig> = OnceCell::const_new();
static CONFIG: LazyLock<RwLock<Arc<AppConfig>>> =
    LazyLock::new(|| RwLock::new(Arc::new(AppConfig::default())));

#[derive(Debug, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default = "ServerConfig::default")]
    server: ServerConfig,
    #[serde(default = "DatabaseConfig::default")]
    database: DatabaseConfig,
    #[serde(default = "AuthConfig::default")]
    auth: AuthConfig,
    #[serde(default = "TenantConfig::default")]
    tenant: TenantConfig,
    #[serde(default = "RedisConfig::default")]
    redis: RedisConfig,
    #[serde(default = "SmsCodeConfig::default")]
    sms_code: SmsCodeConfig,
}

impl AppConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "resources".to_string());
        let mut config_builder = Config::builder()
            .add_source(
                config::File::with_name(format!("{}/application", config_path).as_ref())
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            .add_source(
                config::File::with_name(format!("{}/application-local", config_path).as_ref())
                    .format(FileFormat::Yaml)
                    .required(false),
            );
        let nacos_config = load_nacos_config().await?;
        if nacos_config.is_some() {
            let nacos_config = nacos_config.unwrap();
            config_builder = config_builder.add_source(
                config::File::from_str(nacos_config.content(), FileFormat::Yaml).required(false),
            );
        }
        config_builder
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow!("Failed to parse config"))
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }
    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }
    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
    pub fn tenant(&self) -> &TenantConfig {
        &self.tenant
    }
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }
    pub fn sms_code(&self) -> &SmsCodeConfig {
        &self.sms_code
    }
}
pub async fn refresh() -> anyhow::Result<()> {
    let new_config = AppConfig::load().await?;
    let mut write_guard = CONFIG.write().await;
    *write_guard = Arc::new(new_config);
    Ok(())
}
pub async fn get() -> Arc<AppConfig> {
    CONFIG.read().await.clone()
}

pub fn default_bool() -> bool {
    false
}
