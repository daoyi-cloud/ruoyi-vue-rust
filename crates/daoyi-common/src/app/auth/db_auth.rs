use crate::app::auth::Principal;
use crate::app::enumeration::UserTypeEnum;
use crate::app::utils::is_expired;
use crate::app::{app_redis, database, id};
use daoyi_entities_system::entity::prelude::SystemOauth2AccessToken;
use daoyi_entities_system::entity::system_oauth2_access_token;
use sea_orm::prelude::*;
use std::sync::LazyLock;

static DB_AUTH: LazyLock<DbAuth> = LazyLock::new(|| DbAuth);

pub struct DbAuth;

impl super::Auth for DbAuth {
    async fn encode(&self, _principal: &Principal) -> anyhow::Result<String> {
        Ok(id::x())
    }

    async fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let cache_key = format!("access_token:{}", token);
        let cached = app_redis::cache_get_json::<Principal>(&cache_key).await?;
        if cached.is_some() {
            return Ok(cached.unwrap());
        }
        let at = SystemOauth2AccessToken::find()
            .filter(system_oauth2_access_token::Column::AccessToken.eq(token))
            .one(database::get()?)
            .await?
            .ok_or_else(|| anyhow::anyhow!("访问令牌不存在"))?;
        if is_expired(at.expires_time)? {
            return Err(anyhow::anyhow!("访问令牌已过期"));
        }
        let principal = Principal {
            tenant_id: at.tenant_id,
            user_id: at.user_id,
            user_type: UserTypeEnum::from_value(at.user_type)
                .ok_or_else(|| anyhow::anyhow!("用户类型不存在"))?,
        };
        app_redis::cache_set_json(&cache_key, &principal).await?;
        Ok(principal)
    }
}

pub fn get_default_db_auth() -> &'static DbAuth {
    &DB_AUTH
}
