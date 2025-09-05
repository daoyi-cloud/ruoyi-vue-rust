pub mod auth;
pub mod social;
pub mod user;
pub mod logger;

#[macro_export]
macro_rules! impl_tenant_instance {
    ($s: ident) => {
        impl $s {
            pub fn new(tenant: TenantContextHolder) -> Self {
                Self { tenant }
            }
            pub fn tenant_id(&self) -> i64 {
                self.tenant.tenant_id()
            }
        }
    };
}