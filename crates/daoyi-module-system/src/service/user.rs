use crate::impl_tenant_instance;
use daoyi_common::app::{database, TenantContextHolder};
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::USER_NOT_EXISTS;
use daoyi_entities_system::entity::system_users;
use sea_orm::entity::prelude::*;

pub struct AdminUserService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminUserService);

impl AdminUserService {
    pub async fn get_user_by_username(
        &self,
        username: &str,
    ) -> ApiResult<Option<system_users::Model>> {
        Ok(system_users::Entity::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Username.eq(username))
            .one(database::get()?)
            .await?)
    }
    pub async fn get_user(&self, id: i64) -> ApiResult<system_users::Model> {
        Ok(system_users::Entity::find_by_id(id)
            .one(database::get()?)
            .await?
            .ok_or_else(|| ApiError::BizCode(USER_NOT_EXISTS))?)
    }
}
