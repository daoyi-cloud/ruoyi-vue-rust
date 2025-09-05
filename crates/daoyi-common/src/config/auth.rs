use crate::app::enumeration::AuthMethod;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_method")]
    method: AuthMethod,
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
    pub fn method(&self) -> AuthMethod {
        self.method
    }
    pub fn ignore_urls(&self) -> Vec<String> {
        self.ignore_urls.clone()
    }
}
fn default_method() -> AuthMethod {
    AuthMethod::Jwt
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/demo/api/auth/login")]
}
