use crate::service::auth::AdminAuthService;
use crate::vo::auth::{AuthLoginReqVo, AuthLoginRespVo, AuthPermissionInfoRespVo};
use axum::{Extension, Router, debug_handler, routing};
use daoyi_common::app::{
    AppState,
    auth::Principal,
    error::{ApiJsonResult, api_json_ok},
    valid::ValidJson,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/get-permission-info", routing::get(get_permission_info))
        .route("/login", routing::post(login))
}

#[debug_handler]
async fn login(ValidJson(params): ValidJson<AuthLoginReqVo>) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(AdminAuthService::login(params).await?)
}

#[debug_handler]
async fn get_permission_info(
    Extension(principal): Extension<Principal>,
) -> ApiJsonResult<AuthPermissionInfoRespVo> {
    api_json_ok(AdminAuthService::get_permission_info(principal).await?)
}
