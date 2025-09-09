use crate::vo::auth::AuthRegisterReqVo;
use daoyi_common::app::{TenantContextHolder, database};
use daoyi_common::impl_tenant_instance;
use daoyi_common::service::ConfigApi;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{USER_NOT_EXISTS, USER_REGISTER_DISABLED};
use daoyi_entities_system::entity::prelude::SystemUsers;
use daoyi_entities_system::entity::system_users;
use sea_orm::entity::prelude::*;

pub struct AdminUserService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminUserService);

const USER_INIT_PASSWORD_KEY: &str = "system.user.init-password";

const USER_REGISTER_ENABLED_KEY: &str = "system.user.register-enabled";

impl AdminUserService {
    pub async fn register_user(&self, req_vo: &AuthRegisterReqVo) -> ApiResult<i64> {
        let enable = ConfigApi
            .get_config_value_by_key(USER_REGISTER_ENABLED_KEY)
            .await?
            .ok_or_else(|| ApiError::BizCode(USER_REGISTER_DISABLED))?;
        if enable != "true" {
            return Err(ApiError::BizCode(USER_REGISTER_DISABLED));
        }
        // 1.2 校验账户配合
        let c = SystemUsers::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Deleted.eq(0))
            .count(database::get()?)
            .await?;
        todo!()
    }

    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> ApiResult<Option<system_users::Model>> {
        Ok(SystemUsers::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Deleted.eq(0))
            .filter(system_users::Column::Username.eq(username))
            .one(database::get()?)
            .await?)
    }
    pub async fn get_user(&self, id: i64) -> ApiResult<system_users::Model> {
        Ok(SystemUsers::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Deleted.eq(0))
            .filter(system_users::Column::Id.eq(id))
            .one(database::get()?)
            .await?
            .ok_or_else(|| ApiError::BizCode(USER_NOT_EXISTS))?)
    }
}
