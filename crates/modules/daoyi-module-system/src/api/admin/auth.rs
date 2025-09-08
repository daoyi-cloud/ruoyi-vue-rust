use crate::service::auth::AdminAuthService;
use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use axum::{Extension, Router, debug_handler, routing};
use daoyi_common::app::{AppState, TenantContextHolder, auth::Principal};
use daoyi_common_support::utils::errors::error::{ApiJsonResult, api_json_ok};
use daoyi_common_support::utils::web::valid::ValidJson;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/get-permission-info", routing::get(get_permission_info))
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
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
