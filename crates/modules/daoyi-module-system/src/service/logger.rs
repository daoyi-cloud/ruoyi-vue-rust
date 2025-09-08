use crate::impl_tenant_instance;
use daoyi_common::app::TenantContextHolder;

pub struct LoginLogService {
    tenant: TenantContextHolder,
}
pub struct OperateLogService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(LoginLogService);
impl_tenant_instance!(OperateLogService);
