mod infra_config_service;
pub use infra_config_service::ConfigApi;

#[macro_export]
macro_rules! impl_tenant_instance {
    ($s: ident) => {
        impl $s {
            #[allow(dead_code)]
            pub fn new(tenant: TenantContextHolder) -> Self {
                Self { tenant }
            }
            #[allow(dead_code)]
            fn tenant_id(&self) -> i64 {
                self.tenant.tenant_id()
            }
        }
    };
}
