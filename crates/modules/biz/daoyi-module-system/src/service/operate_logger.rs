use daoyi_common::app::TenantContextHolder;
use daoyi_common::impl_tenant_instance;

pub struct OperateLogService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(OperateLogService);
