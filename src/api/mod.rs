use axum::Router;
use daoyi_common::app::AppState;
use daoyi_common_support::utils::errors::error::{ApiError, ApiResult};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/demo", daoyi_module_demo::api::create_router())
        .nest("/system", daoyi_module_system::api::create_router())
        .nest("/infra", daoyi_module_infra::api::create_router())
        .fallback(async || -> ApiResult<()> {
            tracing::warn!("Not Found");
            Err(ApiError::NotFound)
        })
        .method_not_allowed_fallback(async || -> ApiResult<()> {
            tracing::warn!("Method Not Allowed");
            Err(ApiError::MethodNotAllowed)
        })
}
