mod auth;
mod database;
mod server;

use anyhow::{Context, anyhow};
pub use auth::AuthConfig;
use config::{Config, FileFormat};
pub use database::DatabaseConfig;
use serde::Deserialize;
pub use server::ServerConfig;
use std::sync::LazyLock;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to initialize config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DatabaseConfig,
    #[serde(default = "AuthConfig::default")]
    auth: AuthConfig,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::File::with_name("application-local")
                    .format(FileFormat::Yaml)
                    .required(false),
            )
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
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
