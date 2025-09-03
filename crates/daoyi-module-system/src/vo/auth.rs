use serde::{Deserialize, Serialize};
use daoyi_common::app::enumeration::SocialTypeEnum;

/// AuthLoginReqVO，管理后台 - 账号密码登录 Request VO，如果登录并绑定社交用户，需要传递 social 开头的参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginReqVo {
    /// 验证码，验证码开启时，需要传递
    pub captcha_verification: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 授权码
    pub social_code: Option<String>,
    pub social_code_valid: Option<bool>,
    /// state
    pub social_state: Option<String>,
    /// 社交平台的类型，参见 SocialTypeEnum 枚举值
    pub social_type: Option<SocialTypeEnum>,
    /// 账号
    pub username: Option<String>,
}
