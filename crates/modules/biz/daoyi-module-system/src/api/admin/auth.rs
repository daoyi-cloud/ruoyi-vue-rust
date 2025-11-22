use crate::service::admin_auth::AdminAuthService;
use crate::vo::auth::auth_login_req_vo::AuthLoginReqVo;
use crate::vo::auth::auth_login_resp_vo::AuthLoginRespVo;
use crate::vo::auth::auth_permission_info_resp_vo::AuthPermissionInfoRespVo;
use crate::vo::auth::auth_refresh_token_req_vo::AuthRefreshTokenReqVo;
use crate::vo::auth::auth_register_req_vo::AuthRegisterReqVo;
use crate::vo::auth::auth_sms_send_req_vo::AuthSmsSendReqVo;
use axum::extract::ConnectInfo;
use axum::{Extension, Router, debug_handler, routing};
use daoyi_common::app::{AppState, TenantContextHolder, auth::Principal};
use daoyi_common_support::utils::web::valid::{ValidJson, ValidQuery};
use daoyi_common_support::utils::{
    errors::error::{ApiJsonResult, api_json_ok},
    web::response::ApiJsonResponse,
};
use std::net::SocketAddr;
use utoipa::OpenApi;

/// 认证模块 OpenAPI 文档
#[derive(OpenApi)]
#[openapi(
    paths(
        login,
        logout,
        refresh_token,
        get_permission_info,
        register,
        send_sms_code,
    ),
    components(
        schemas(
            AuthLoginReqVo,
            AuthLoginRespVo,
            AuthRefreshTokenReqVo,
            AuthRegisterReqVo,
            AuthSmsSendReqVo,
            AuthPermissionInfoRespVo,
        )
    ),
    tags(
        (name = "auth", description = "认证管理 API")
    )
)]
pub struct AuthApiDoc;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/login", routing::post(login))
        .route("/logout", routing::post(logout))
        .route("/refresh-token", routing::post(refresh_token))
        .route("/get-permission-info", routing::get(get_permission_info))
        .route("/register", routing::post(register))
        .route("/send-sms-code", routing::post(send_sms_code))
}

#[utoipa::path(
    post,
    path = "/admin-api/auth/send-sms-code",
    tag = "auth",
    summary = "发送短信验证码",
    description = "发送短信验证码用于注册或登录",
    request_body = AuthSmsSendReqVo,
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx")
    ),
    responses(
        (status = 200, description = "发送成功", body = ApiJsonResponse<bool>),
        (status = 400, description = "参数错误"),
        (status = 429, description = "发送过于频繁"),
    ),
    security(
        ("tenant_id" = [])
    )
)]
#[debug_handler]
async fn send_sms_code(
    Extension(ConnectInfo(addr)): Extension<ConnectInfo<SocketAddr>>,
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthSmsSendReqVo>,
) -> ApiJsonResult<bool> {
    AdminAuthService::new(tenant)
        .send_sms_code(params, addr.ip().to_string())
        .await?;
    api_json_ok(true)
}

#[utoipa::path(
    post,
    path = "/admin-api/auth/register",
    tag = "auth",
    summary = "用户注册",
    description = "注册新用户账号",
    request_body = AuthRegisterReqVo,
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx")
    ),
    responses(
        (status = 200, description = "注册成功", body = ApiJsonResponse<AuthLoginRespVo>),
        (status = 400, description = "注册参数错误"),
    ),
    security(
        ("tenant_id" = [])
    )
)]
#[debug_handler]
async fn register(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthRegisterReqVo>,
) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(AdminAuthService::new(tenant).register(params).await?)
}

#[utoipa::path(
    get,
    path = "/admin-api/auth/get-permission-info",
    tag = "auth",
    summary = "获取权限信息",
    description = "获取当前用户的权限信息",
    security(
        ("tenant_id" = []),
        ("bearer_auth" = [])
    ),
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx")
    ),
    responses(
        (status = 200, description = "获取成功", body = ApiJsonResponse<AuthPermissionInfoRespVo>),
        (status = 401, description = "未授权"),
    )
)]
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

#[utoipa::path(
    post,
    path = "/admin-api/auth/refresh-token",
    tag = "auth",
    summary = "刷新访问令牌",
    description = "使用刷新令牌获取新的访问令牌",
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx"),
        ("refreshToken" = String, Query, description = "刷新令牌")
    ),
    responses(
        (status = 200, description = "刷新成功", body = ApiJsonResponse<AuthLoginRespVo>),
        (status = 400, description = "刷新令牌无效"),
    ),
    security(
        ("tenant_id" = [])
    )
)]
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

#[utoipa::path(
    post,
    path = "/admin-api/auth/logout",
    tag = "auth",
    summary = "登出系统",
    description = "退出当前登录会话",
    security(
        ("tenant_id" = []),
        ("bearer_auth" = [])
    ),
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx")
    ),
    responses(
        (status = 200, description = "登出成功"),
        (status = 401, description = "未授权"),
    )
)]
#[debug_handler]
async fn logout(
    Extension(tenant): Extension<TenantContextHolder>,
    Extension(principal): Extension<Principal>,
) -> ApiJsonResult<()> {
    api_json_ok(AdminAuthService::new(tenant).logout(principal).await?)
}

#[utoipa::path(
    post,
    path = "/admin-api/auth/login",
    tag = "auth",
    summary = "管理员登录",
    description = "使用账号密码登录管理后台",
    request_body = AuthLoginReqVo,
    params(
        ("tenant-id" = String, Header, description = "租户ID，例如 1", example = "1"),
        ("Authorization" = Option<String>, Header, description = "Bearer Token，格式如 `Bearer <token>`，可选", example = "Bearer xxx")
    ),
    responses(
        (status = 200, description = "登录成功", body = ApiJsonResponse<AuthLoginRespVo>),
        (status = 400, description = "请求参数错误"),
        (status = 401, description = "账号或密码错误"),
    ),
    security(
        ("tenant_id" = [])
    )
)]
#[debug_handler]
async fn login(
    Extension(tenant): Extension<TenantContextHolder>,
    ValidJson(params): ValidJson<AuthLoginReqVo>,
) -> ApiJsonResult<AuthLoginRespVo> {
    api_json_ok(AdminAuthService::new(tenant).login(params).await?)
}
