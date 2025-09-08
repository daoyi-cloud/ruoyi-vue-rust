use daoyi_common_support::utils::enumeration::UserTypeEnum;
use daoyi_common_support::utils::errors::error::ApiResult;
use serde::{Deserialize, Serialize};

pub mod db_auth;
pub mod jsonwebtoken_auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub tenant_id: i64,
    pub user_id: i64,
    pub user_type: UserTypeEnum,
    pub token: String,
}

pub trait Auth {
    fn encode(&self, principal: &Principal) -> impl Future<Output = ApiResult<String>>;
    fn decode(&self, token: &str) -> impl Future<Output = ApiResult<Principal>>;
}
