use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantContextHolder {
    tenant_id: i64,
    ignore: bool,
}

impl TenantContextHolder {
    pub fn set_tenant_id(tenant_id: i64) -> Self {
        Self {
            tenant_id,
            ignore: false,
        }
    }
}

impl Default for TenantContextHolder {
    fn default() -> Self {
        Self {
            tenant_id: 0,
            ignore: true,
        }
    }
}
