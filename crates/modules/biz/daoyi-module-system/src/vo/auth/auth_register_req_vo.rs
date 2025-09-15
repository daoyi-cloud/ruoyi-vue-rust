use daoyi_common_support::utils::web::validation::validate_username;
use daoyi_entities_system::entity::system_users;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// AuthRegisterReqVO，管理后台 - Register Request VO
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthRegisterReqVo {
    /// 验证码，验证码开启时，需要传递
    pub captcha_verification: Option<String>,
    /// 用户昵称
    #[validate(length(max = 30, message = "用户昵称长度不能超过 30 个字符"))]
    pub nickname: String,
    /// 密码
    #[validate(length(min = 4, max = 16, message = "密码长度为 4-16 位"))]
    pub password: String,
    /// 用户账号
    #[validate(custom(function = "validate_username"))]
    #[validate(length(min = 4, max = 30, message = "用户账号长度为 4-30 个字符"))]
    pub username: String,
}

impl From<AuthRegisterReqVo> for system_users::ActiveModel {
    fn from(value: AuthRegisterReqVo) -> Self {
        Self {
            username: Set(value.username),
            nickname: Set(value.nickname),
            password: Set(value.password),
            ..Default::default()
        }
    }
}
