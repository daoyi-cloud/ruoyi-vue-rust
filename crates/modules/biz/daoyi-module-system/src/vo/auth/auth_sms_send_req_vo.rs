use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// AuthSmsSendReqVO，管理后台 - 发送手机验证码 Request VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthSmsSendReqVo {
    /// 验证码，验证码开启时，需要传递
    pub captcha_verification: Option<String>,
    /// 手机号
    #[schema(example = "13800138000")]
    pub mobile: String,
    /// 短信场景
    #[schema(example = 1)]
    pub scene: i32,
}
