use crate::service::dept::DeptService;
use crate::service::post::PostService;
use crate::service::tenant::TenantService;
use crate::vo::auth::AuthRegisterReqVo;
use daoyi_common::app::{TenantContextHolder, database};
use daoyi_common::impl_tenant_instance;
use daoyi_common::service::ConfigApi;
use daoyi_common_support::support::orm::create_with_common_fields;
use daoyi_common_support::utils::encode_password;
use daoyi_common_support::utils::enumeration::CommonStatusEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{
    USER_COUNT_MAX, USER_EMAIL_EXISTS, USER_MOBILE_EXISTS, USER_NOT_EXISTS, USER_REGISTER_DISABLED,
    USER_USERNAME_EXISTS,
};
use daoyi_entities_system::entity::prelude::SystemUsers;
use daoyi_entities_system::entity::system_users;
use sea_orm::Set;
use sea_orm::entity::prelude::*;

pub struct AdminUserService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminUserService);
impl AdminUserService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemUsers> {
        SystemUsers::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Deleted.eq(0))
    }
}

const _USER_INIT_PASSWORD_KEY: &str = "system.user.init-password";

const USER_REGISTER_ENABLED_KEY: &str = "system.user.register-enabled";

impl AdminUserService {
    pub async fn get_user_by_mobile(&self, mobile: &str) -> ApiResult<Option<system_users::Model>> {
        Ok(self
            .base_query()
            .filter(system_users::Column::Mobile.eq(mobile))
            .one(database::get()?)
            .await?)
    }
    pub async fn register_user(&self, req_vo: &AuthRegisterReqVo) -> ApiResult<i64> {
        let enable = ConfigApi
            .get_config_value_by_key(USER_REGISTER_ENABLED_KEY)
            .await?
            .ok_or_else(|| ApiError::BizCode(USER_REGISTER_DISABLED))?;
        if enable != "true" {
            return Err(ApiError::BizCode(USER_REGISTER_DISABLED));
        }
        // 1.2 校验账户配合
        let user_count = self.base_query().count(database::get()?).await?;
        let tenant = TenantService::new(self.tenant.clone())
            .get_current()
            .await?;
        if user_count >= tenant.account_count as u64 {
            return Err(ApiError::BizCodeWithArgs(
                USER_COUNT_MAX,
                vec![format!("{}", tenant.account_count)],
            ));
        }
        // 1.3 校验正确性
        self.validate_user_for_create_or_update(
            None,
            req_vo.username.as_ref(),
            "",
            "",
            None,
            vec![],
        )
        .await?;
        // 2. 插入用户
        let active_model = system_users::ActiveModel::from(req_vo.clone());
        let mut active_model = create_with_common_fields(active_model, None, &self.tenant).await?;
        active_model.status = Set(CommonStatusEnum::Enable.status());
        active_model.password = Set(encode_password(req_vo.password.as_str())?);
        let user_id = active_model.insert(database::get()?).await?.id;
        Ok(user_id)
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> ApiResult<Option<system_users::Model>> {
        Ok(self
            .base_query()
            .filter(system_users::Column::Username.eq(username))
            .one(database::get()?)
            .await?)
    }

    pub async fn get_user(&self, id: i64) -> ApiResult<system_users::Model> {
        Ok(self
            .base_query()
            .filter(system_users::Column::Id.eq(id))
            .one(database::get()?)
            .await?
            .ok_or_else(|| ApiError::BizCode(USER_NOT_EXISTS))?)
    }

    pub async fn validate_user_for_create_or_update(
        &self,
        id: Option<i64>,
        username: &str,
        mobile: &str,
        email: &str,
        dept_id: Option<i64>,
        post_ids: Vec<i64>,
    ) -> ApiResult<Option<system_users::Model>> {
        // 校验用户存在
        let user = self.validate_user_exists(id).await?;
        // 校验用户名唯一
        self.validate_username_unique(id, username).await?;
        // 校验手机号唯一
        self.validate_mobile_unique(id, mobile).await?;
        // 校验邮箱唯一
        self.validate_email_unique(id, email).await?;
        // 校验部门处于开启状态
        if dept_id.is_some() {
            DeptService::new(self.tenant.clone())
                .validate_dept_list(vec![dept_id.unwrap()])
                .await?;
        }
        // 校验岗位处于开启状态
        PostService::new(self.tenant.clone())
            .validate_post_list(post_ids)
            .await?;
        Ok(user)
    }

    pub async fn validate_email_unique(&self, id: Option<i64>, email: &str) -> ApiResult<()> {
        if email.is_empty() {
            return Ok(());
        }
        let user = self
            .base_query()
            .filter(system_users::Column::Email.eq(email))
            .one(database::get()?)
            .await?;
        if user.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(ApiError::BizCode(USER_EMAIL_EXISTS));
        }
        if user.unwrap().id != id.unwrap() {
            return Err(ApiError::BizCode(USER_EMAIL_EXISTS));
        }
        Ok(())
    }

    pub async fn validate_mobile_unique(&self, id: Option<i64>, mobile: &str) -> ApiResult<()> {
        if mobile.is_empty() {
            return Ok(());
        }
        let user = self
            .base_query()
            .filter(system_users::Column::Mobile.eq(mobile))
            .one(database::get()?)
            .await?;
        if user.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(ApiError::BizCode(USER_MOBILE_EXISTS));
        }
        if user.unwrap().id != id.unwrap() {
            return Err(ApiError::BizCode(USER_MOBILE_EXISTS));
        }
        Ok(())
    }

    pub async fn validate_username_unique(&self, id: Option<i64>, username: &str) -> ApiResult<()> {
        if username.is_empty() {
            return Ok(());
        }
        let user = self
            .base_query()
            .filter(system_users::Column::Username.eq(username))
            .one(database::get()?)
            .await?;
        if user.is_none() {
            return Ok(());
        }
        // 如果 id 为空，说明不用比较是否为相同 id 的用户
        if id.is_none() {
            return Err(ApiError::BizCode(USER_USERNAME_EXISTS));
        }
        if user.unwrap().id != id.unwrap() {
            return Err(ApiError::BizCode(USER_USERNAME_EXISTS));
        }
        Ok(())
    }

    pub async fn validate_user_exists(
        &self,
        id: Option<i64>,
    ) -> ApiResult<Option<system_users::Model>> {
        if id.is_none() {
            return Ok(None);
        }
        Ok(Some(self.get_user(id.unwrap()).await?))
    }
}
