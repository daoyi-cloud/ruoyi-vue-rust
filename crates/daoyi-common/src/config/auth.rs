use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_ignore_urls")]
    ignore_urls: Vec<String>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            ignore_urls: default_ignore_urls(),
        }
    }
}

impl AuthConfig {
    pub fn ignore_urls(&self) -> Vec<String> {
        self.ignore_urls.clone()
    }
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/demo/api/auth/login")]
}
