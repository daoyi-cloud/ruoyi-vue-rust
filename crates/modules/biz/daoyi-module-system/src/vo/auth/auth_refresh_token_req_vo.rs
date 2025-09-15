use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuthRefreshTokenReqVo {
    /// 刷新令牌
    pub refresh_token: String,
}
