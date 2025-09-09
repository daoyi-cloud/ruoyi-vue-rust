use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::errors::error::ApiResult;
use daoyi_entities_system::entity::prelude::SystemRole;
use daoyi_entities_system::entity::system_role;
use sea_orm::*;

pub struct RoleService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(RoleService);
impl RoleService {
    pub async fn get_role_list(
        &self,
        role_ids: impl IntoIterator<Item = i64>,
    ) -> ApiResult<Vec<system_role::Model>> {
        Ok(SystemRole::find()
            .filter(system_role::Column::Id.is_in(role_ids))
            .all(database::get()?)
            .await?)
    }
}
