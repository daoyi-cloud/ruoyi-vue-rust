use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AuthRefreshTokenReqVo {
    /// 刷新令牌
    #[schema(example = "refresh_token_xxx")]
    pub refresh_token: String,
}
