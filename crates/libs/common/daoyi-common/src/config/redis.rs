use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct RedisConfig {
    host: Option<String>,
    port: Option<u16>,
    password: Option<String>,
    database: Option<u8>,
    cache_key_prefix: Option<String>,
    expire_seconds: Option<u64>,
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

    pub fn cache_key_prefix(&self) -> &str {
        self.cache_key_prefix
            .as_deref()
            .unwrap_or("app:ruoyi-vue-rust")
    }

    pub fn expire_seconds(&self) -> u64 {
        self.expire_seconds.unwrap_or(60)
    }
}
