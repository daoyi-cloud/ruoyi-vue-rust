use crate::impl_tenant_instance;
use crate::service::user::AdminUserService;
use daoyi_common::app::{TenantContextHolder, database, redis_util};
use daoyi_common::security::login_user::LoginUser;
use daoyi_common_support::support::orm::create_with_common_fields;
use daoyi_common_support::utils;
use daoyi_common_support::utils::enumeration::{
    CommonStatusEnum, EMPTY_VEC_STR, UserTypeEnum, redis_key_constants::OAUTH_CLIENT,
    redis_key_constants::OAUTH2_ACCESS_TOKEN,
};
use daoyi_common_support::utils::errors::{
    OAUTH2_CLIENT_AUTHORIZED_GRANT_TYPE_NOT_EXISTS, OAUTH2_CLIENT_CLIENT_SECRET_ERROR,
    OAUTH2_CLIENT_DISABLE, OAUTH2_CLIENT_NOT_EXISTS, OAUTH2_CLIENT_REDIRECT_URI_NOT_MATCH,
    OAUTH2_CLIENT_SCOPE_OVER,
    error::{ApiError, ApiResult},
};
use daoyi_common_support::utils::is_expired;
use daoyi_entities_system::entity::prelude::{SystemOauth2AccessToken, SystemOauth2RefreshToken};
use daoyi_entities_system::entity::{
    prelude::SystemOauth2Client, system_oauth2_access_token, system_oauth2_client,
    system_oauth2_refresh_token,
};
use sea_orm::sqlx::types::chrono::Local;
use sea_orm::{prelude::*, *};
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;

pub struct OAuth2TokenService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(OAuth2TokenService);
pub struct OAuth2ClientService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(OAuth2ClientService);

impl OAuth2TokenService {
    pub async fn refresh_access_token(
        &self,
        refresh_token: String,
        client_id: &str,
    ) -> ApiResult<system_oauth2_access_token::Model> {
        let db = database::get()?;
        // 查询访问令牌
        let refresh_token = SystemOauth2RefreshToken::find()
            .filter(system_oauth2_refresh_token::Column::RefreshToken.eq(refresh_token))
            .one(db)
            .await?
            .ok_or_else(|| ApiError::Validation(String::from("无效的刷新令牌")))?;
        // 校验 Client 匹配
        let client = OAuth2ClientService::new(self.tenant.clone())
            .valid_oauth_client_from_cache(client_id)
            .await?;
        // 移除相关的访问令牌
        let access_tokens = SystemOauth2AccessToken::find()
            .filter(
                system_oauth2_access_token::Column::RefreshToken.eq(&refresh_token.refresh_token),
            )
            .all(db)
            .await?;
        for access_token in access_tokens {
            redis_util::del(&format!(
                "{OAUTH2_ACCESS_TOKEN}:{}",
                access_token.access_token
            ))
            .await?;
            SystemOauth2AccessToken::delete_by_id(access_token.id)
                .exec(db)
                .await?;
        }
        // 已过期的情况下，删除刷新令牌
        if is_expired(refresh_token.expires_time)? {
            SystemOauth2RefreshToken::delete_by_id(refresh_token.id)
                .exec(db)
                .await?;
            return Err(ApiError::Unauthenticated(String::from("刷新令牌已过期")));
        }
        // 创建访问令牌
        self.create_oauth2access_token(&refresh_token, &client)
            .await
    }

    pub async fn remove_access_token(&self, access_token: &str) -> ApiResult<()> {
        let db = database::get()?;
        let token = SystemOauth2AccessToken::find()
            .filter(system_oauth2_access_token::Column::AccessToken.eq(access_token))
            .one(db)
            .await?;
        if token.is_none() {
            return Ok(());
        }
        let token = token.unwrap();
        redis_util::del(&format!("{OAUTH2_ACCESS_TOKEN}:{access_token}")).await?;
        SystemOauth2AccessToken::delete_many()
            .filter(system_oauth2_access_token::Column::AccessToken.eq(access_token))
            .exec(db)
            .await?;
        SystemOauth2RefreshToken::delete_many()
            .filter(system_oauth2_refresh_token::Column::RefreshToken.eq(token.refresh_token))
            .exec(db)
            .await?;
        Ok(())
    }
    pub async fn create_access_token(
        &self,
        user_id: i64,
        user_type: i32,
        client_id: &str,
        scopes: Vec<String>,
    ) -> ApiResult<system_oauth2_access_token::Model> {
        let client = OAuth2ClientService::new(self.tenant.clone())
            .valid_oauth_client_from_cache(client_id)
            .await?;
        let refresh_token = self
            .create_oauth2refresh_token(user_id, user_type, &client, scopes)
            .await?;
        self.create_oauth2access_token(&refresh_token, &client)
            .await
    }

    async fn create_oauth2access_token(
        &self,
        refresh_token: &system_oauth2_refresh_token::Model,
        client: &system_oauth2_client::Model,
    ) -> ApiResult<system_oauth2_access_token::Model> {
        let active_model = system_oauth2_access_token::ActiveModel {
            access_token: Set(utils::id::x()),
            user_id: Set(refresh_token.user_id),
            user_type: Set(refresh_token.user_type),
            user_info: Set(self
                .build_user_info(refresh_token.user_id, refresh_token.user_type)
                .await?),
            client_id: Set(client.client_id.to_owned()),
            scopes: Set(refresh_token.scopes.to_owned()),
            refresh_token: Set(refresh_token.refresh_token.to_owned()),
            expires_time: Set(Local::now()
                .add(Duration::from_secs(
                    client.access_token_validity_seconds as u64,
                ))
                .naive_local()),
            ..Default::default()
        };
        let model = create_with_common_fields(
            active_model,
            Some(refresh_token.user_id.to_string()),
            &self.tenant,
        )
        .await?
        .insert(database::get()?)
        .await?;
        if client.access_token_validity_seconds > 0 {
            redis_util::cache_set_json_ex(
                &format!("{OAUTH2_ACCESS_TOKEN}:{}", model.access_token),
                &model,
                client.access_token_validity_seconds as u64,
            )
            .await?;
        }
        Ok(model)
    }

    async fn create_oauth2refresh_token(
        &self,
        user_id: i64,
        user_type: i32,
        client: &system_oauth2_client::Model,
        scopes: Vec<String>,
    ) -> ApiResult<system_oauth2_refresh_token::Model> {
        let active_model = system_oauth2_refresh_token::ActiveModel {
            refresh_token: Set(utils::id::x()),
            user_id: Set(user_id),
            user_type: Set(user_type),
            client_id: Set(client.client_id.to_owned()),
            scopes: Set(Some(serde_json::to_string(&scopes)?)),
            expires_time: Set(Local::now()
                .add(Duration::from_secs(
                    client.refresh_token_validity_seconds as u64,
                ))
                .naive_local()),
            ..Default::default()
        };
        let model =
            create_with_common_fields(active_model, Some(user_id.to_string()), &self.tenant)
                .await?
                .insert(database::get()?)
                .await?;
        Ok(model)
    }

    async fn build_user_info(&self, user_id: i64, user_type: i32) -> ApiResult<String> {
        let mut user_info = HashMap::<&str, String>::new();
        if UserTypeEnum::is_admin(user_type) {
            let user = AdminUserService::new(self.tenant.clone())
                .get_user(user_id)
                .await?;
            user_info.insert(LoginUser::INFO_KEY_NICKNAME, user.nickname);
            user_info.insert(
                LoginUser::INFO_KEY_DEPT_ID,
                match user.dept_id {
                    None => String::new(),
                    Some(dept_id) => dept_id.to_string(),
                },
            );
        } else if UserTypeEnum::is_member(user_type) {
        } else {
            return Err(ApiError::Biz(format!("未知用户类型: {}", user_type)));
        }
        let result = serde_json::to_string(&user_info)?;
        Ok(result)
    }
}

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
