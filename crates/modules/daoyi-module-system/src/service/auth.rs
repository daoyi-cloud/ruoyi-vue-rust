use crate::service::menu::MenuService;
use crate::service::oauth2::OAuth2TokenService;
use crate::service::permission::PermissionService;
use crate::service::role::RoleService;
use crate::service::sms::SmsCodeApi;
use crate::service::user::AdminUserService;
use crate::vo::auth::{
    AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo, AuthRegisterReqVo, AuthSmsSendReqVo,
    SmsCodeSendReqDTO, UserVo,
};
use daoyi_common::app::TenantContextHolder;
use daoyi_common::app::auth::Principal;
use daoyi_common::{config, impl_tenant_instance};
use daoyi_common_support::utils::enumeration::{
    CommonStatusEnum, SmsSceneEnum, UserTypeEnum, oauth2_client_constants,
};
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};
use daoyi_common_support::utils::errors::{
    AUTH_LOGIN_BAD_CREDENTIALS, AUTH_LOGIN_USER_DISABLED, AUTH_MOBILE_NOT_EXISTS,
    AUTH_REGISTER_CAPTCHA_CODE_ERROR,
};
use daoyi_common_support::utils::{RANDOM_PASSWORD, enumeration, verify_password};
use daoyi_entities_system::entity::system_users;

pub struct AdminAuthService {
    tenant: TenantContextHolder,
}
impl_tenant_instance!(AdminAuthService);
impl AdminAuthService {
    pub async fn send_sms_code(
        &self,
        req_vo: AuthSmsSendReqVo,
        create_ip: String,
    ) -> ApiResult<()> {
        // 如果是重置密码场景，需要校验图形验证码是否正确
        if SmsSceneEnum::AdminMemberResetPassword.scene() == req_vo.scene {
            self.validate_captcha(req_vo.captcha_verification.as_deref())
                .await?;
        }
        // 登录场景，验证是否存在
        AdminUserService::new(self.tenant.clone())
            .get_user_by_mobile(req_vo.mobile.as_ref())
            .await?
            .ok_or_else(|| ApiError::BizCode(AUTH_MOBILE_NOT_EXISTS))?;
        // 发送验证码
        SmsCodeApi::new(self.tenant.clone())
            .send_sms_code(SmsCodeSendReqDTO {
                mobile: req_vo.mobile,
                scene: req_vo.scene,
                create_ip,
            })
            .await?;
        Ok(())
    }
    pub async fn register(&self, req_vo: AuthRegisterReqVo) -> ApiResult<AuthLoginRespVo> {
        // 1. 校验验证码
        self.validate_captcha(req_vo.captcha_verification.as_deref())
            .await?;
        // 2. 校验用户名是否已存在
        let user_id = AdminUserService::new(self.tenant.clone())
            .register_user(&req_vo)
            .await?;
        self.create_token_after_login_success(
            user_id,
            &req_vo.username,
            enumeration::LoginLogTypeEnum::LoginUsername,
        )
        .await
    }

    async fn validate_captcha(&self, captcha_verification: Option<&str>) -> ApiResult<()> {
        if config::get().auth().captcha() {
            if captcha_verification.is_none() {
                return Err(ApiError::BizCodeWithArgs(
                    AUTH_REGISTER_CAPTCHA_CODE_ERROR,
                    vec![String::from("验证码不能为空")],
                ));
            }
            return Err(ApiError::BizCodeWithArgs(
                AUTH_REGISTER_CAPTCHA_CODE_ERROR,
                vec![String::from("未实现验证逻辑")],
            ));
        }
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: String) -> ApiResult<AuthLoginRespVo> {
        let token = OAuth2TokenService::new(self.tenant.clone())
            .refresh_access_token(refresh_token, oauth2_client_constants::CLIENT_ID_DEFAULT)
            .await?;
        // 构建返回结果
        Ok(token.into())
    }

    pub async fn logout(&self, principal: Principal) -> ApiResult<()> {
        OAuth2TokenService::new(self.tenant.clone())
            .remove_access_token(&principal.token)
            .await
    }

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
        principal: Principal,
    ) -> ApiResult<AuthPermissionInfoRespVo> {
        let mut vo = AuthPermissionInfoRespVo::default();
        // 1.1 获得用户信息
        let user = AdminUserService::new(self.tenant.clone())
            .get_user(principal.user_id)
            .await?;
        vo.user = UserVo::from(user);
        let permission_service = PermissionService::new(self.tenant.clone());
        // 1.2 获得角色列表
        let role_ids = permission_service
            .get_user_role_id_list_by_user_id(principal.user_id)
            .await?;
        if role_ids.is_empty() {
            return Ok(vo);
        }
        let roles = RoleService::new(self.tenant.clone())
            .get_role_list(role_ids)
            .await?
            .into_iter()
            .filter(|role| CommonStatusEnum::is_enable(role.status))
            .collect::<Vec<_>>();
        let role_ids = roles.iter().map(|item| item.id).collect::<Vec<_>>();
        vo.roles = roles.into_iter().map(|item| item.code).collect();
        // 1.3 获得菜单列表
        let menu_ids = permission_service
            .get_role_menu_list_by_role_id(role_ids, &vo.roles)
            .await?;
        let menus = MenuService::new(self.tenant.clone())
            .get_menu_list(menu_ids)
            .await?
            .into_iter()
            .filter(|item| CommonStatusEnum::is_enable(item.status))
            .collect::<Vec<_>>();
        vo.permissions = menus
            .clone()
            .into_iter()
            .map(|item| item.permission)
            .collect();
        vo.menus = MenuService::build_tree(menus).await?;
        Ok(vo)
    }
}
