use crate::app::{auth::Principal, database, redis_util};
use daoyi_common_support::utils;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_entities_system::entity::{prelude::SystemOauth2AccessToken, system_oauth2_access_token};
use sea_orm::prelude::*;
use std::sync::LazyLock;

static DB_AUTH: LazyLock<DbAuth> = LazyLock::new(|| DbAuth);

pub struct DbAuth;

impl super::Auth for DbAuth {
    async fn encode(&self, _principal: &Principal) -> ApiResult<String> {
        Ok(utils::id::x())
    }

    async fn decode(&self, token: &str) -> ApiResult<Principal> {
        let cache_key = format!("access_token:{}", token);
        let cached = redis_util::cache_get_json::<Principal>(&cache_key).await?;
        if cached.is_some() {
            return Ok(cached.unwrap());
        }
        let at = SystemOauth2AccessToken::find()
            .filter(system_oauth2_access_token::Column::AccessToken.eq(token))
            .one(database::get()?)
            .await?
            .ok_or_else(|| ApiError::Unauthenticated(String::from("访问令牌不存在")))?;
        if utils::is_expired(&at.expires_time)? {
            if crate::config::get().await.auth().auto_renew() {
                // 自动续期,即为永不过期，但是数据暂时不改变，前端拿到的Token看起来还是保持过期状态
            } else {
                return Err(ApiError::Unauthenticated(String::from("访问令牌已过期")));
            }
        }
        let principal = Principal {
            tenant_id: at.tenant_id,
            user_id: at.user_id,
            user_type: utils::enumeration::UserTypeEnum::from_value(at.user_type)
                .ok_or_else(|| ApiError::Unauthenticated(String::from("用户类型不存在")))?,
            token: String::from(token),
        };
        redis_util::cache_set_json(&cache_key, &principal).await?;
        Ok(principal)
    }
}

pub fn get_default_db_auth() -> &'static DbAuth {
    &DB_AUTH
}
