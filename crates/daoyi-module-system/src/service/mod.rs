pub mod auth;
pub mod social;
pub mod user;
pub mod logger;
mod oauth2;

#[macro_export]
macro_rules! impl_tenant_instance {
    ($s: ident) => {
        impl $s {
            #[allow(dead_code)]
            pub fn new(tenant: TenantContextHolder) -> Self {
                Self { tenant }
            }
            #[allow(dead_code)]
            pub fn tenant_id(&self) -> i64 {
                self.tenant.tenant_id()
            }
        }
    };
}