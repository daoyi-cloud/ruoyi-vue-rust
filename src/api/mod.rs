use crate::app::{
    AppState,
    error::{ApiError, ApiResult},
};
use axum::Router;

mod auth;
mod user;

pub fn create_router() -> Router<AppState> {
    Router::new().nest(
        "/api",
        Router::new()
            .nest("/users", user::create_router())
            .nest("/auth", auth::create_router())
            .fallback(async || -> ApiResult<()> {
                tracing::warn!("Not Found");
                Err(ApiError::NotFound)
            })
            .method_not_allowed_fallback(async || -> ApiResult<()> {
                tracing::warn!("Method Not Allowed");
                Err(ApiError::MethodNotAllowed)
            }),
    )
}

// 错误示范（违反孤儿规则）
// impl IntoResponse for anyhow::Error {
//     fn into_response(self) -> axum::response::Response {
//         tracing::error!("{:?}", self);
//         (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
//     }
// }
