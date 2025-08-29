use axum::Router;
use daoyi_common::app::AppState;
use daoyi_common::app::error::{ApiError, ApiResult};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/demo", daoyi_module_demo::api::create_router())
        .fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
