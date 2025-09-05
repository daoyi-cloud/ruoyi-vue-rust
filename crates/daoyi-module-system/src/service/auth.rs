use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use daoyi_common::app::TenantContextHolder;
use daoyi_common::app::auth::Principal;

pub struct AdminAuthService {
    tenant: TenantContextHolder,
}
impl AdminAuthService {
    pub fn new(tenant: TenantContextHolder) -> Self {
        Self { tenant }
    }
    pub fn tenant_id(&self) -> i64 {
        self.tenant.tenant_id()
    }
}
impl AdminAuthService {
    pub async fn login(&self, _req_vo: AuthLoginReqVo) -> anyhow::Result<AuthLoginRespVo> {
        Ok(AuthLoginRespVo {
            access_token: "".to_string(),
            expires_time: "".to_string(),
            refresh_token: "".to_string(),
            user_id: 0,
        })
    }
    pub async fn get_permission_info(
        &self,
        _principal: Principal,
    ) -> anyhow::Result<AuthPermissionInfoRespVo> {
        Ok(AuthPermissionInfoRespVo::default())
    }
}
