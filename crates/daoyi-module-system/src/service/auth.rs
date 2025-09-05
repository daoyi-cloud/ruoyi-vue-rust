use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use daoyi_common::app::auth::Principal;

pub struct AdminAuthService;
impl AdminAuthService {
    pub async fn login(_req_vo: AuthLoginReqVo) -> anyhow::Result<AuthLoginRespVo> {
        Ok(AuthLoginRespVo {
            access_token: "".to_string(),
            expires_time: "".to_string(),
            refresh_token: "".to_string(),
            user_id: 0,
        })
    }
    pub async fn get_permission_info(
        _principal: Principal,
    ) -> anyhow::Result<AuthPermissionInfoRespVo> {
        Ok(AuthPermissionInfoRespVo::default())
    }
}
