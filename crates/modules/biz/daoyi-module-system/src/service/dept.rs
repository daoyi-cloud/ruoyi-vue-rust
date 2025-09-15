use daoyi_common::app::database;
use daoyi_common::impl_tenant_instance;
use daoyi_common_support::support::tenant::TenantContextHolder;
use daoyi_common_support::utils::enumeration::CommonStatusEnum;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{DEPT_NOT_ENABLE, DEPT_NOT_FOUND};
use daoyi_entities_system::entity::prelude::SystemDept;
use daoyi_entities_system::entity::system_dept;
use sea_orm::*;

pub struct DeptService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(DeptService);
impl DeptService {
    // 提取公共查询条件到基础方法
    fn base_query(&self) -> Select<SystemDept> {
        SystemDept::find()
            .filter(system_dept::Column::TenantId.eq(self.tenant_id()))
            .filter(system_dept::Column::Deleted.eq(0))
    }
}
impl DeptService {
    pub async fn validate_dept_list(&self, dept_ids: Vec<i64>) -> ApiResult<()> {
        if dept_ids.is_empty() {
            return Ok(());
        }
        let len = dept_ids.len();
        let dept_list = self
            .base_query()
            .filter(system_dept::Column::Id.is_in(dept_ids))
            .all(database::get()?)
            .await?;
        if dept_list.len() != len {
            return Err(ApiError::BizCode(DEPT_NOT_FOUND));
        }
        for d in dept_list {
            if CommonStatusEnum::is_disable(d.status) {
                return Err(ApiError::BizCodeWithArgs(DEPT_NOT_ENABLE, vec![d.name]));
            }
        }
        Ok(())
    }
}
