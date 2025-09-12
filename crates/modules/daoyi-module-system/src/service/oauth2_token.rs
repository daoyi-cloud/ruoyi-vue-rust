use crate::service::admin_user::AdminUserService;
use crate::service::oauth2_client::OAuth2ClientService;
use daoyi_common::app::{TenantContextHolder, database, redis_util};
use daoyi_common::impl_tenant_instance;
use daoyi_common::security::login_user::LoginUser;
use daoyi_common_support::support::orm::create_with_common_fields;
use daoyi_common_support::utils;
use daoyi_common_support::utils::enumeration::{
    UserTypeEnum, redis_key_constants::OAUTH2_ACCESS_TOKEN,
};
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::is_expired;
use daoyi_entities_system::entity::prelude::{SystemOauth2AccessToken, SystemOauth2RefreshToken};
use daoyi_entities_system::entity::{
    system_oauth2_access_token, system_oauth2_client, system_oauth2_refresh_token,
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
            .ok_or_else(|| ApiError::InvalidRefreshToken)?;
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
        if is_expired(&refresh_token.expires_time)? {
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
