use daoyi_common_support::utils::serde::datetime_format;
use daoyi_entities_system::entity::system_oauth2_access_token;
use sea_orm::prelude::DateTime;
use serde::Serialize;
use utoipa::ToSchema;

/// AuthLoginRespVO，管理后台 - 登录 Response VO
#[derive(Debug, Clone, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthLoginRespVo {
    /// 访问令牌
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    /// 过期时间
    #[serde(with = "datetime_format")]
    #[schema(value_type = String, example = "2024-12-31 23:59:59")]
    pub expires_time: DateTime,
    /// 刷新令牌
    #[schema(example = "refresh_token_xxx")]
    pub refresh_token: String,
    /// 用户编号
    #[schema(example = 1)]
    pub user_id: i64,
}

impl From<system_oauth2_access_token::Model> for AuthLoginRespVo {
    fn from(value: system_oauth2_access_token::Model) -> Self {
        Self {
            access_token: value.access_token,
            expires_time: value.expires_time,
            refresh_token: value.refresh_token,
            user_id: value.user_id,
        }
    }
}
