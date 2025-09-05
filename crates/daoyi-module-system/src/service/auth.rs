use crate::impl_tenant_instance;
use crate::service::user::AdminUserService;
use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use daoyi_common::app::TenantContextHolder;
use daoyi_common::app::auth::Principal;
use daoyi_common::app::enumeration::CommonStatusEnum;
use daoyi_common::app::errors::error::{ApiError, ApiResult};
use daoyi_common::app::errors::{AUTH_LOGIN_BAD_CREDENTIALS, AUTH_LOGIN_USER_DISABLED};
use daoyi_common::app::utils::{RANDOM_PASSWORD, verify_password};
use daoyi_entities_system::entity::system_users;

pub struct AdminAuthService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminAuthService);
impl AdminAuthService {
    pub async fn login(&self, req_vo: AuthLoginReqVo) -> anyhow::Result<AuthLoginRespVo> {
        let user = self.authenticate(&req_vo.username, &req_vo.password).await?;
        Ok(AuthLoginRespVo {
            access_token: "".to_string(),
            expires_time: "".to_string(),
            refresh_token: "".to_string(),
            user_id: 0,
        })
    }

    pub async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> ApiResult<system_users::Model> {
        let user = AdminUserService::new(self.tenant.clone())
            .get_user_by_username(username)
            .await?;
        if user.is_none() {
            // 模拟校验密码，密码错误，模拟耗时
            let _ = verify_password(password, &RANDOM_PASSWORD);
            return Err(ApiError::BizCode(AUTH_LOGIN_BAD_CREDENTIALS));
        }
        let user = user.unwrap();
        let result = verify_password(password, &user.password)?;
        if !result {
            return Err(ApiError::BizCode(AUTH_LOGIN_BAD_CREDENTIALS));
        }
        if CommonStatusEnum::is_disable(user.status) {
            return Err(ApiError::BizCode(AUTH_LOGIN_USER_DISABLED));
        }
        Ok(user)
    }
    pub async fn get_permission_info(
        &self,
        _principal: Principal,
    ) -> anyhow::Result<AuthPermissionInfoRespVo> {
        Ok(AuthPermissionInfoRespVo::default())
    }
}
