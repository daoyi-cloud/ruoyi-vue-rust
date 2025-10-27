use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// AuthLoginReqVO，管理后台 - 账号密码登录 Request VO，如果登录并绑定社交用户，需要传递 social 开头的参数
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginReqVo {
    /// 验证码，验证码开启时，需要传递
    #[schema(example = "1234")]
    pub captcha_verification: Option<String>,
    /// 密码
    #[schema(example = "admin123")]
    pub password: String,
    /// 授权码
    pub social_code: Option<String>,
    pub social_code_valid: Option<bool>,
    /// state
    pub social_state: Option<String>,
    /// 社交平台的类型，参见 SocialTypeEnum 枚举值
    pub social_type: Option<i32>,
    /// 账号
    #[schema(example = "admin")]
    pub username: String,
}
