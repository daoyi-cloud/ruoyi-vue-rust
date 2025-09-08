use daoyi_common_support::utils::enumeration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_method")]
    method: enumeration::AuthMethod,
    #[serde(default = "default_ignore_urls")]
    ignore_urls: Vec<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: default_method(),
            ignore_urls: default_ignore_urls(),
        }
    }
}

impl AuthConfig {
    pub fn method(&self) -> enumeration::AuthMethod {
        self.method
    }
    pub fn ignore_urls(&self) -> Vec<String> {
        self.ignore_urls.clone()
    }
}
fn default_method() -> enumeration::AuthMethod {
    enumeration::AuthMethod::Jwt
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/demo/api/auth/login")]
}
