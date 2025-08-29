use daoyi_common::app::{
    AppState,
    auth::{Principal, get_jwt},
    error::{ApiError, ApiJsonResult, api_json_msg_ok, api_json_ok},
    middleware::get_auth_layer,
    utils::{RANDOM_PASSWORD, verify_password},
    valid::ValidJson,
};
use daoyi_common::entity::{prelude::*, sys_user};
use axum::{Extension, Router, debug_handler, extract::State, routing};
use sea_orm::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/user-info", routing::get(get_user_info))
        .route_layer(get_auth_layer())
        .route("/login", routing::post(login))
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 1, max = 16, message = "账号长度1-16"))]
    account: String,
    #[validate(length(min = 6, max = 16, message = "密码长度6-16"))]
    password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    access_token: String,
}

#[tracing::instrument(name = "login", skip_all, fields(account = %params.account))]
#[debug_handler]
async fn login(
    State(AppState { db }): State<AppState>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiJsonResult<LoginResult> {
    tracing::info!("开始处理登录逻辑...");
    let user = SysUser::find()
        .filter(sys_user::Column::Account.eq(&params.account))
        .one(&db)
        .await?
        .ok_or_else(|| {
            tracing::error!("用户不存在");
            // 模拟校验密码，密码错误，模拟耗时
            let _ = verify_password(&params.password, &RANDOM_PASSWORD);
            ApiError::Biz(String::from("账号或密码不正确"))
        })?;
    if !verify_password(&params.password, &user.password)? {
        tracing::error!("密码错误");
        return Err(ApiError::Biz(String::from("账号或密码不正确")));
    }
    let principal = Principal {
        id: user.id,
        name: user.name,
    };
    let access_token = get_jwt().encode(&principal)?;
    tracing::info!("登录成功...JWT token: {access_token}");
    api_json_msg_ok("登录成功", LoginResult { access_token })
}

#[debug_handler]
async fn get_user_info(Extension(principal): Extension<Principal>) -> ApiJsonResult<Principal> {
    api_json_ok(principal)
}
