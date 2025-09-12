use daoyi_common::app::{TenantContextHolder, database, redis_util};
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::utils::enumeration::{
    CommonStatusEnum, EMPTY_VEC_STR, redis_key_constants::OAUTH_CLIENT,
};
use daoyi_common_support::utils::errors::{
    OAUTH2_CLIENT_AUTHORIZED_GRANT_TYPE_NOT_EXISTS, OAUTH2_CLIENT_CLIENT_SECRET_ERROR,
    OAUTH2_CLIENT_DISABLE, OAUTH2_CLIENT_NOT_EXISTS, OAUTH2_CLIENT_REDIRECT_URI_NOT_MATCH,
    OAUTH2_CLIENT_SCOPE_OVER,
    error::{ApiError, ApiResult},
};
use daoyi_entities_system::entity::{prelude::SystemOauth2Client, system_oauth2_client};
use sea_orm::prelude::*;

pub struct OAuth2ClientService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(OAuth2ClientService);

impl OAuth2ClientService {
    /**
     * 获得 OAuth2 客户端，从缓存中
     *
     * @param clientId 客户端编号
     * @return OAuth2 客户端
     */
    pub async fn get_oauth2client_from_cache(
        &self,
        client_id: &str,
    ) -> ApiResult<Option<system_oauth2_client::Model>> {
        let cache_key = format!("{OAUTH_CLIENT}:{client_id}");
        let cached = redis_util::cache_get_json::<system_oauth2_client::Model>(&cache_key).await?;
        if cached.is_some() {
            return Ok(cached);
        }
        let client = SystemOauth2Client::find()
            .filter(system_oauth2_client::Column::ClientId.eq(client_id))
            .one(database::get()?)
            .await?;
        if client.is_some() {
            redis_util::cache_set_json(&cache_key, &client).await?;
        }
        Ok(client)
    }
    /**
     * 从缓存中，校验客户端是否合法
     *
     * @return 客户端
     */
    pub async fn valid_oauth_client_from_cache(
        &self,
        client_id: &str,
    ) -> ApiResult<system_oauth2_client::Model> {
        self.valid_oauth_client_from_cache2(client_id, "", "", vec![], "")
            .await
    }

    /**
     * 从缓存中，校验客户端是否合法
     *
     * 非空时，进行校验
     *
     * @param clientId 客户端编号
     * @param clientSecret 客户端密钥
     * @param authorizedGrantType 授权方式
     * @param scopes 授权范围
     * @param redirectUri 重定向地址
     * @return 客户端
     */
    pub async fn valid_oauth_client_from_cache2(
        &self,
        client_id: &str,
        client_secret: &str,
        authorized_grant_type: &str,
        scopes: Vec<&str>,
        redirect_uri: &str,
    ) -> ApiResult<system_oauth2_client::Model> {
        // 校验客户端存在、且开启
        let client = self
            .get_oauth2client_from_cache(client_id)
            .await?
            .ok_or_else(|| ApiError::BizCode(OAUTH2_CLIENT_NOT_EXISTS))?;
        if CommonStatusEnum::is_disable(client.status) {
            return Err(ApiError::BizCode(OAUTH2_CLIENT_DISABLE));
        }
        // 校验客户端密钥
        if !client_secret.is_empty() && client_secret != client.secret {
            return Err(ApiError::BizCode(OAUTH2_CLIENT_CLIENT_SECRET_ERROR));
        }
        // 校验授权方式
        let client_agt = serde_json::from_str::<Vec<&str>>(
            client
                .authorized_grant_types
                .as_deref()
                .unwrap_or_else(|| EMPTY_VEC_STR),
        )?;
        if !authorized_grant_type.is_empty() && !client_agt.contains(&authorized_grant_type) {
            return Err(ApiError::BizCode(
                OAUTH2_CLIENT_AUTHORIZED_GRANT_TYPE_NOT_EXISTS,
            ));
        }
        // 校验授权范围
        let client_scopes = serde_json::from_str::<Vec<&str>>(
            client.scopes.as_deref().unwrap_or_else(|| EMPTY_VEC_STR),
        )?;
        if !scopes.is_empty() && !scopes.iter().all(|scope| client_scopes.contains(scope)) {
            return Err(ApiError::BizCode(OAUTH2_CLIENT_SCOPE_OVER));
        }
        // 校验回调地址
        let client_redirect_uris = serde_json::from_str::<Vec<&str>>(
            client
                .redirect_uris
                .as_deref()
                .unwrap_or_else(|| EMPTY_VEC_STR),
        )?;
        if !redirect_uri.is_empty()
            && !client_redirect_uris
                .iter()
                .any(|uri| redirect_uri.starts_with(uri))
        {
            return Err(ApiError::BizCode(OAUTH2_CLIENT_REDIRECT_URI_NOT_MATCH));
        }
        Ok(client)
    }
}
