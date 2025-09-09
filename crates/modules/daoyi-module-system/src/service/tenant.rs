use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::errors::TENANT_NOT_EXISTS;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_entities_system::entity::prelude::SystemTenant;
use daoyi_entities_system::entity::system_tenant;
use sea_orm::EntityTrait;

pub struct TenantService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(TenantService);

impl TenantService {
    pub async fn get_current(&self) -> ApiResult<system_tenant::Model> {
        Ok(SystemTenant::find_by_id(self.tenant_id())
            .one(database::get()?)
            .await?
            .ok_or_else(|| ApiError::BizCode(TENANT_NOT_EXISTS))?)
    }
}
