use crate::service::auth::AdminAuthService;
use crate::vo::auth::{
    AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo, AuthRefreshTokenReqVo,
    AuthRegisterReqVo, AuthSmsSendReqVo,
};
use axum::{Extension, Router, debug_handler, routing};
use daoyi_common::app::{AppState, TenantContextHolder, auth::Principal};
use daoyi_common_support::utils::errors::error::{ApiJsonResult, api_json_ok};
use daoyi_common_support::utils::web::valid::{ValidJson, ValidQuery};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
        .route("/refresh-token", routing::post(refresh_token))
        .route("/get-permission-info", routing::get(get_permission_info))
        .route("/register", routing::post(register))
        .route("//send-sms-code", routing::post(send_sms_code))
}

#[debug_handler]
async fn send_sms_code(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthSmsSendReqVo>,
) -> ApiJsonResult<bool> {
    AdminAuthService::new(tenant).send_sms_code(params).await?;
    api_json_ok(true)
}

#[debug_handler]
async fn register(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthRegisterReqVo>,
) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(AdminAuthService::new(tenant).register(params).await?)
}

#[debug_handler]
async fn get_permission_info(
    Extension(tenant): Extension<TenantContextHolder>,
    Extension(principal): Extension<Principal>,
) -> ApiJsonResult<AuthPermissionInfoRespVo> {
    api_json_ok(
        AdminAuthService::new(tenant)
            .get_permission_info(principal)
            .await?,
    )
}

#[debug_handler]
async fn refresh_token(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidQuery(params): ValidQuery<AuthRefreshTokenReqVo>,
) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(
        AdminAuthService::new(tenant)
            .refresh_token(params.refresh_token)
            .await?,
    )
}

#[debug_handler]
async fn logout(
    Extension(tenant): Extension<TenantContextHolder>,
    Extension(principal): Extension<Principal>,
) -> ApiJsonResult<()> {
    api_json_ok(AdminAuthService::new(tenant).logout(principal).await?)
}

#[debug_handler]
async fn login(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthLoginReqVo>,
) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(AdminAuthService::new(tenant).login(params).await?)
}
