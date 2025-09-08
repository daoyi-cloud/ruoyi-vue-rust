use crate::impl_tenant_instance;
use daoyi_common::app::TenantContextHolder;

pub struct SocialUserService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(SocialUserService);
