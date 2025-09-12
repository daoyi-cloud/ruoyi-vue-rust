use daoyi_common::app::TenantContextHolder;
use daoyi_common::impl_tenant_instance;

pub struct LoginLogService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(LoginLogService);
