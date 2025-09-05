use crate::app::enumeration::UserTypeEnum;
use serde::Serialize;

pub mod jsonwebtoken_auth;

#[derive(Debug, Clone, Serialize)]
pub struct Principal {
    pub tenant_id: i64,
    pub user_id: i64,
    pub user_type: UserTypeEnum,
}
