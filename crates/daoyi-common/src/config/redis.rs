use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>,
    database: Option<u8>,
}

impl RedisConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(6379)
    }
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("")
    }
    pub fn database(&self) -> u8 {
        self.database.unwrap_or(0u8)
    }
}
