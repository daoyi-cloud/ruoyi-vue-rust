use crate::config::default_bool;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TenantConfig {
    #[serde(default = "default_bool")]
    enabled: bool,
    #[serde(default = "default_ignore_urls")]
    ignore_urls: Vec<String>,
    #[serde(default = "default_header_name")]
    header_name: String,
}

impl Default for TenantConfig {
    fn default() -> Self {
        Self {
            enabled: default_bool(),
            ignore_urls: default_ignore_urls(),
            header_name: default_header_name(),
        }
    }
}

impl TenantConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn ignore_urls(&self) -> Vec<&str> {
        self.ignore_urls.iter().map(String::as_str).collect()
    }
    pub fn header_name(&self) -> &str {
        self.header_name.as_ref()
    }
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/demo/api/auth/login")]
}
fn default_header_name() -> String {
    String::from("X-Tenant-Id")
}
