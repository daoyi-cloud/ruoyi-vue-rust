use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TenantContextHolder {
    tenant_id: i64,
    ignore: bool,
}

impl TenantContextHolder {
    pub fn ignore(&self) -> bool {
        self.ignore
    }
    pub fn tenant_id(&self) -> i64 {
        self.tenant_id
    }
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
