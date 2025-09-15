use daoyi_common_support::utils::enumeration;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    #[serde(default = "default_method")]
    method: enumeration::AuthMethod,
    #[serde(default = "default_ignore_urls")]
    ignore_urls: Vec<String>,
    /// Token是否自动续期
    #[serde(default = "default_auto_renew")]
    auto_renew: bool,
    /// 验证码开关
    #[serde(default = "default_captcha")]
    captcha: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: default_method(),
            ignore_urls: default_ignore_urls(),
            auto_renew: default_auto_renew(),
            captcha: default_captcha(),
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
    pub fn auto_renew(&self) -> bool {
        self.auto_renew
    }
    pub fn captcha(&self) -> bool {
        self.captcha
    }
}
fn default_method() -> enumeration::AuthMethod {
    enumeration::AuthMethod::Jwt
}

fn default_ignore_urls() -> Vec<String> {
    vec![String::from("/demo/api/auth/login")]
}

fn default_auto_renew() -> bool {
    true
}

fn default_captcha() -> bool {
    false
}
