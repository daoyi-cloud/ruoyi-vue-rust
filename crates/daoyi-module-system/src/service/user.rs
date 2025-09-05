use crate::impl_tenant_instance;
use daoyi_common::app::{TenantContextHolder, database};
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
    ) -> anyhow::Result<Option<system_users::Model>> {
        Ok(system_users::Entity::find()
            .filter(system_users::Column::TenantId.eq(self.tenant_id()))
            .filter(system_users::Column::Username.eq(username))
            .one(database::get()?)
            .await?)
    }
}
