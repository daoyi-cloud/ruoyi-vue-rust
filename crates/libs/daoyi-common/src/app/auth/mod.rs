use daoyi_common_support::utils::enumeration::UserTypeEnum;
use serde::{Deserialize, Serialize};

pub mod db_auth;
pub mod jsonwebtoken_auth;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principal {
    pub tenant_id: i64,
    pub user_id: i64,
    pub user_type: UserTypeEnum,
}

pub trait Auth {
    fn encode(&self, principal: &Principal) -> impl Future<Output = anyhow::Result<String>>;
    fn decode(&self, token: &str) -> impl Future<Output = anyhow::Result<Principal>>;
}
