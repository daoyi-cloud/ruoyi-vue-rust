use axum::Router;
use daoyi_common::app::AppState;
use daoyi_common_support::utils::errors::error::api_json_ok;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            axum::routing::get(|| async { api_json_ok("Hello, Ruoyi Vue Rust!") }),
        )
        .nest("/demo", daoyi_module_demo::api::create_router())
        .nest("/system", daoyi_module_system::api::create_router())
        .nest("/infra", daoyi_module_infra::api::create_router())
}
