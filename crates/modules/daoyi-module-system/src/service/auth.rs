use crate::impl_tenant_instance;
use crate::service::oauth2::OAuth2TokenService;
use crate::service::user::AdminUserService;
use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use daoyi_common::app::TenantContextHolder;
use daoyi_common::app::auth::Principal;
use daoyi_common_support::utils::enumeration::{UserTypeEnum, oauth2_client_constants};
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{AUTH_LOGIN_BAD_CREDENTIALS, AUTH_LOGIN_USER_DISABLED};
use daoyi_common_support::utils::{RANDOM_PASSWORD, enumeration, verify_password};
use daoyi_entities_system::entity::system_users;

pub struct AdminAuthService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminAuthService);
impl AdminAuthService {
    pub async fn login(&self, req_vo: AuthLoginReqVo) -> ApiResult<AuthLoginRespVo> {
        let user = self
            .authenticate(&req_vo.username, &req_vo.password)
            .await?;
        self.create_token_after_login_success(
            user.id,
            &user.username,
            enumeration::LoginLogTypeEnum::LoginUsername,
        )
        .await
    }

    pub async fn create_token_after_login_success(
        &self,
        user_id: i64,
        _username: &str,
        _login_type: enumeration::LoginLogTypeEnum,
    ) -> ApiResult<AuthLoginRespVo> {
        // 插入登陆日志
        // 创建访问令牌
        let token = OAuth2TokenService::new(self.tenant.clone())
            .create_access_token(
                user_id,
                UserTypeEnum::Admin.value(),
                oauth2_client_constants::CLIENT_ID_DEFAULT,
                vec![],
            )
            .await?;
        // 构建返回结果
        Ok(token.into())
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
        if enumeration::CommonStatusEnum::is_disable(user.status) {
            return Err(ApiError::BizCode(AUTH_LOGIN_USER_DISABLED));
        }
        Ok(user)
    }
    pub async fn get_permission_info(
        &self,
        _principal: Principal,
    ) -> ApiResult<AuthPermissionInfoRespVo> {
        Ok(AuthPermissionInfoRespVo::default())
    }
}
